#pragma once

#if defined(_WIN32)
#include_next <bcrypt.h>
#else

#include <cstddef>
#include <cstdint>
#include <cstring>
#include <cwchar>
#include <sys/random.h>
#include <unistd.h>
#include <openssl/evp.h>
#include <openssl/hmac.h>
#include <openssl/rand.h>

#define BCRYPT_USE_SYSTEM_PREFERRED_RNG 0x00000002
#define BCRYPT_SUCCESS(status) (((int)(status)) >= 0)
#define STATUS_SUCCESS 0
#define BCRYPT_ALG_HANDLE_HMAC_FLAG 0x00000008

#define BCRYPT_AES_ALGORITHM L"AES"
#define BCRYPT_SHA256_ALGORITHM L"SHA256"
#define BCRYPT_CHAINING_MODE L"ChainingMode"
#define BCRYPT_CHAIN_MODE_GCM L"ChainingModeGCM"
#define BCRYPT_CHAIN_MODE_CBC L"ChainingModeCBC"

typedef int NTSTATUS;
typedef unsigned char UCHAR;
typedef unsigned char* PUCHAR;
typedef unsigned long ULONG;
typedef unsigned long long ULONGLONG;
typedef void* BCRYPT_ALG_HANDLE;
typedef void* BCRYPT_KEY_HANDLE;
typedef void* BCRYPT_HASH_HANDLE;

typedef struct _BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO {
    ULONG cbSize;
    ULONG dwInfoVersion;
    PUCHAR pbNonce;
    ULONG cbNonce;
    PUCHAR pbAuthData;
    ULONG cbAuthData;
    PUCHAR pbTag;
    ULONG cbTag;
    PUCHAR pbMacContext;
    ULONG cbMacContext;
    ULONG cbAAD;
    unsigned long long cbData;
    ULONG dwFlags;
} BCRYPT_AUTHENTICATED_CIPHER_MODE_INFO, *PBCRYPT_AUTHENTICATED_CIPHER_MODE_INFO;

#define BCRYPT_INIT_AUTH_MODE_INFO(info) \
    do { \
        std::memset(&(info), 0, sizeof(info)); \
        (info).cbSize = sizeof(info); \
        (info).dwInfoVersion = 1; \
    } while (0)

struct ShimAlgorithm {
    enum Type { AES_GCM, AES_CBC, SHA256, HMAC_SHA256 } type;
};

struct ShimKey {
    ShimAlgorithm::Type type;
    unsigned char key[64];
    size_t keyLen;
};

struct ShimHash {
    ShimAlgorithm::Type type;
    EVP_MD_CTX* ctx;
    EVP_MAC_CTX* macCtx;
};

inline NTSTATUS BCryptOpenAlgorithmProvider(BCRYPT_ALG_HANDLE* phAlgorithm, const wchar_t* pszAlgId, const wchar_t*, unsigned long dwFlags) {
    if (!phAlgorithm) return -1;
    ShimAlgorithm* alg = new ShimAlgorithm();
    if (wcscmp(pszAlgId, BCRYPT_AES_ALGORITHM) == 0) {
        alg->type = ShimAlgorithm::AES_GCM;
    } else if (wcscmp(pszAlgId, BCRYPT_SHA256_ALGORITHM) == 0) {
        alg->type = (dwFlags & BCRYPT_ALG_HANDLE_HMAC_FLAG) ? ShimAlgorithm::HMAC_SHA256 : ShimAlgorithm::SHA256;
    } else {
        alg->type = ShimAlgorithm::AES_GCM;
    }
    *phAlgorithm = (BCRYPT_ALG_HANDLE)alg;
    return STATUS_SUCCESS;
}

inline NTSTATUS BCryptSetProperty(BCRYPT_ALG_HANDLE hObject, const wchar_t* pszProperty, PUCHAR pbInput, ULONG, unsigned long) {
    if (!hObject) return -1;
    ShimAlgorithm* alg = (ShimAlgorithm*)hObject;
    if (wcscmp(pszProperty, BCRYPT_CHAINING_MODE) == 0) {
        const wchar_t* mode = (const wchar_t*)pbInput;
        if (wcscmp(mode, BCRYPT_CHAIN_MODE_GCM) == 0) {
            alg->type = ShimAlgorithm::AES_GCM;
        } else if (wcscmp(mode, BCRYPT_CHAIN_MODE_CBC) == 0) {
            alg->type = ShimAlgorithm::AES_CBC;
        }
    }
    return STATUS_SUCCESS;
}

inline NTSTATUS BCryptCloseAlgorithmProvider(BCRYPT_ALG_HANDLE hAlgorithm, unsigned long) {
    if (hAlgorithm) {
        delete (ShimAlgorithm*)hAlgorithm;
    }
    return STATUS_SUCCESS;
}

inline NTSTATUS BCryptGenerateSymmetricKey(BCRYPT_ALG_HANDLE hAlgorithm, BCRYPT_KEY_HANDLE* phKey, PUCHAR, ULONG, PUCHAR pbSecret, ULONG cbSecret, unsigned long) {
    if (!hAlgorithm || !phKey || !pbSecret) return -1;
    ShimAlgorithm* alg = (ShimAlgorithm*)hAlgorithm;
    ShimKey* key = new ShimKey();
    key->type = alg->type;
    key->keyLen = cbSecret < sizeof(key->key) ? cbSecret : sizeof(key->key);
    std::memcpy(key->key, pbSecret, key->keyLen);
    *phKey = (BCRYPT_KEY_HANDLE)key;
    return STATUS_SUCCESS;
}

inline NTSTATUS BCryptDestroyKey(BCRYPT_KEY_HANDLE hKey) {
    if (hKey) {
        delete (ShimKey*)hKey;
    }
    return STATUS_SUCCESS;
}

inline NTSTATUS BCryptEncrypt(BCRYPT_KEY_HANDLE hKey, PUCHAR pbInput, ULONG cbInput, void* pPaddingInfo, PUCHAR pbIV, ULONG cbIV, PUCHAR pbOutput, ULONG cbOutput, ULONG* pcbResult, unsigned long) {
    if (!hKey) return -1;
    ShimKey* key = (ShimKey*)hKey;

    if (key->type == ShimAlgorithm::AES_GCM) {
        PBCRYPT_AUTHENTICATED_CIPHER_MODE_INFO auth = (PBCRYPT_AUTHENTICATED_CIPHER_MODE_INFO)pPaddingInfo;
        if (!auth) return -1;
        EVP_CIPHER_CTX* ctx = EVP_CIPHER_CTX_new();
        if (!ctx) return -1;
        if (!EVP_EncryptInit_ex(ctx, EVP_aes_128_gcm(), nullptr, nullptr, nullptr)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        if (!EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_IVLEN, auth->cbNonce, nullptr)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        if (!EVP_EncryptInit_ex(ctx, nullptr, nullptr, key->key, auth->pbNonce)) { EVP_CIPHER_CTX_free(ctx); return -1; }

        int outlen = 0;
        if (cbInput > 0 && !EVP_EncryptUpdate(ctx, pbOutput, &outlen, pbInput, cbInput)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        int total = outlen;
        if (!EVP_EncryptFinal_ex(ctx, pbOutput + total, &outlen)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        total += outlen;

        if (!EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_GET_TAG, auth->cbTag, auth->pbTag)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        EVP_CIPHER_CTX_free(ctx);
        if (pcbResult) *pcbResult = total;
        return STATUS_SUCCESS;
    } else if (key->type == ShimAlgorithm::AES_CBC) {
        EVP_CIPHER_CTX* ctx = EVP_CIPHER_CTX_new();
        if (!ctx) return -1;
        if (!EVP_EncryptInit_ex(ctx, EVP_aes_128_cbc(), nullptr, key->key, pbIV)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        EVP_CIPHER_CTX_set_padding(ctx, 1);
        int outlen = 0;
        if (!EVP_EncryptUpdate(ctx, pbOutput, &outlen, pbInput, cbInput)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        int total = outlen;
        if (!EVP_EncryptFinal_ex(ctx, pbOutput + total, &outlen)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        total += outlen;
        EVP_CIPHER_CTX_free(ctx);
        if (pcbResult) *pcbResult = total;
        return STATUS_SUCCESS;
    }
    return -1;
}

inline NTSTATUS BCryptDecrypt(BCRYPT_KEY_HANDLE hKey, PUCHAR pbInput, ULONG cbInput, void* pPaddingInfo, PUCHAR pbIV, ULONG, PUCHAR pbOutput, ULONG, ULONG* pcbResult, unsigned long) {
    if (!hKey) return -1;
    ShimKey* key = (ShimKey*)hKey;

    if (key->type == ShimAlgorithm::AES_GCM) {
        PBCRYPT_AUTHENTICATED_CIPHER_MODE_INFO auth = (PBCRYPT_AUTHENTICATED_CIPHER_MODE_INFO)pPaddingInfo;
        if (!auth) return -1;
        EVP_CIPHER_CTX* ctx = EVP_CIPHER_CTX_new();
        if (!ctx) return -1;
        if (!EVP_DecryptInit_ex(ctx, EVP_aes_128_gcm(), nullptr, nullptr, nullptr)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        if (!EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_IVLEN, auth->cbNonce, nullptr)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        if (!EVP_DecryptInit_ex(ctx, nullptr, nullptr, key->key, auth->pbNonce)) { EVP_CIPHER_CTX_free(ctx); return -1; }

        int outlen = 0;
        if (cbInput > 0 && !EVP_DecryptUpdate(ctx, pbOutput, &outlen, pbInput, cbInput)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        int total = outlen;

        if (!EVP_CIPHER_CTX_ctrl(ctx, EVP_CTRL_GCM_SET_TAG, auth->cbTag, auth->pbTag)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        if (!EVP_DecryptFinal_ex(ctx, pbOutput + total, &outlen)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        total += outlen;
        EVP_CIPHER_CTX_free(ctx);
        if (pcbResult) *pcbResult = total;
        return STATUS_SUCCESS;
    } else if (key->type == ShimAlgorithm::AES_CBC) {
        EVP_CIPHER_CTX* ctx = EVP_CIPHER_CTX_new();
        if (!ctx) return -1;
        if (!EVP_DecryptInit_ex(ctx, EVP_aes_128_cbc(), nullptr, key->key, pbIV)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        EVP_CIPHER_CTX_set_padding(ctx, 1);
        int outlen = 0;
        if (!EVP_DecryptUpdate(ctx, pbOutput, &outlen, pbInput, cbInput)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        int total = outlen;
        if (!EVP_DecryptFinal_ex(ctx, pbOutput + total, &outlen)) { EVP_CIPHER_CTX_free(ctx); return -1; }
        total += outlen;
        EVP_CIPHER_CTX_free(ctx);
        if (pcbResult) *pcbResult = total;
        return STATUS_SUCCESS;
    }
    return -1;
}

inline NTSTATUS BCryptCreateHash(BCRYPT_ALG_HANDLE hAlgorithm, BCRYPT_HASH_HANDLE* phHash, PUCHAR, ULONG, PUCHAR pbSecret, ULONG cbSecret, unsigned long) {
    if (!hAlgorithm || !phHash) return -1;
    ShimAlgorithm* alg = (ShimAlgorithm*)hAlgorithm;
    ShimHash* hash = new ShimHash();
    hash->type = alg->type;
    hash->ctx = nullptr;
    hash->macCtx = nullptr;

    if (hash->type == ShimAlgorithm::SHA256) {
        hash->ctx = EVP_MD_CTX_new();
        if (!hash->ctx || !EVP_DigestInit_ex(hash->ctx, EVP_sha256(), nullptr)) {
            if (hash->ctx) EVP_MD_CTX_free(hash->ctx);
            delete hash;
            return -1;
        }
    } else if (hash->type == ShimAlgorithm::HMAC_SHA256) {
        hash->ctx = EVP_MD_CTX_new();
        // HMAC implementation using HMAC_CTX or EVP
        EVP_PKEY* pkey = EVP_PKEY_new_mac_key(EVP_PKEY_HMAC, nullptr, pbSecret, cbSecret);
        if (!pkey || !EVP_DigestSignInit(hash->ctx, nullptr, EVP_sha256(), nullptr, pkey)) {
            if (pkey) EVP_PKEY_free(pkey);
            if (hash->ctx) EVP_MD_CTX_free(hash->ctx);
            delete hash;
            return -1;
        }
        EVP_PKEY_free(pkey);
    }
    *phHash = (BCRYPT_HASH_HANDLE)hash;
    return STATUS_SUCCESS;
}

inline NTSTATUS BCryptHashData(BCRYPT_HASH_HANDLE hHash, PUCHAR pbInput, ULONG cbInput, unsigned long) {
    if (!hHash || !pbInput) return -1;
    ShimHash* hash = (ShimHash*)hHash;
    if (hash->type == ShimAlgorithm::SHA256) {
        if (!EVP_DigestUpdate(hash->ctx, pbInput, cbInput)) return -1;
    } else if (hash->type == ShimAlgorithm::HMAC_SHA256) {
        if (!EVP_DigestSignUpdate(hash->ctx, pbInput, cbInput)) return -1;
    }
    return STATUS_SUCCESS;
}

inline NTSTATUS BCryptFinishHash(BCRYPT_HASH_HANDLE hHash, PUCHAR pbOutput, ULONG cbOutput, unsigned long) {
    if (!hHash || !pbOutput) return -1;
    ShimHash* hash = (ShimHash*)hHash;
    if (hash->type == ShimAlgorithm::SHA256) {
        unsigned int len = 0;
        if (!EVP_DigestFinal_ex(hash->ctx, pbOutput, &len)) return -1;
    } else if (hash->type == ShimAlgorithm::HMAC_SHA256) {
        size_t len = cbOutput;
        if (!EVP_DigestSignFinal(hash->ctx, pbOutput, &len)) return -1;
    }
    return STATUS_SUCCESS;
}

inline NTSTATUS BCryptDestroyHash(BCRYPT_HASH_HANDLE hHash) {
    if (hHash) {
        ShimHash* hash = (ShimHash*)hHash;
        if (hash->ctx) EVP_MD_CTX_free(hash->ctx);
        delete hash;
    }
    return STATUS_SUCCESS;
}

inline NTSTATUS BCryptGenRandom(BCRYPT_ALG_HANDLE, unsigned char* pbBuffer, unsigned long cbBuffer, unsigned long) {
    if (RAND_bytes(pbBuffer, cbBuffer) == 1) return STATUS_SUCCESS;
    ssize_t res = getrandom(pbBuffer, cbBuffer, 0);
    return res == (ssize_t)cbBuffer ? STATUS_SUCCESS : -1;
}

#endif
