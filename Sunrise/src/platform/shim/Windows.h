#pragma once

#if defined(_WIN32)
#include_next <Windows.h>
#else

#include <cstddef>
#include <cstdint>
#include <cstdio>
#include <cstdlib>
#include <cstring>
#include <cwchar>
#include <pthread.h>
#include <sys/random.h>
#include <sys/stat.h>
#include <sys/types.h>
#include <fcntl.h>
#include <unistd.h>
#include <dirent.h>
#include <errno.h>
#include <time.h>

#define WINAPI
#define STDMETHODCALLTYPE
#define __cdecl
#define __fastcall
#define __stdcall
#define __declspec(x)
#define _ReturnAddress() __builtin_return_address(0)

#define TRUE 1
#define FALSE 0
#define NO_ERROR 0L
#define INVALID_HANDLE_VALUE ((void*)(intptr_t)-1)
#define MAX_PATH 260
#define VK_INSERT 0x2D

#define GENERIC_READ 0x80000000
#define GENERIC_WRITE 0x40000000
#define FILE_SHARE_READ 0x00000001
#define FILE_SHARE_WRITE 0x00000002
#define FILE_SHARE_DELETE 0x00000004
#define CREATE_NEW 1
#define CREATE_ALWAYS 2
#define OPEN_EXISTING 3
#define FILE_ATTRIBUTE_NORMAL 0x00000080
#define FILE_ATTRIBUTE_DIRECTORY 0x00000010
#define FILE_FLAG_RANDOM_ACCESS 0x10000000
#define FILE_FLAG_SEQUENTIAL_SCAN 0x08000000
#define FILE_BEGIN 0
#define FILE_CURRENT 1
#define FILE_END 2

#define ERROR_SUCCESS 0
#define ERROR_FILE_NOT_FOUND 2
#define ERROR_PATH_NOT_FOUND 3
#define ERROR_NO_MORE_FILES 18
#define ERROR_FILE_EXISTS 80
#define ERROR_ALREADY_EXISTS 183
#define MOVEFILE_REPLACE_EXISTING 0x00000001
#define MOVEFILE_WRITE_THROUGH 0x00000008
#define INFINITE 0xFFFFFFFF
#define INVALID_FILE_ATTRIBUTES ((DWORD)-1)

#define IMAGE_DOS_SIGNATURE 0x5A4D
#define IMAGE_NT_SIGNATURE 0x00004550
#define IMAGE_NT_OPTIONAL_HDR64_MAGIC 0x20B

typedef int BOOL;
typedef unsigned int UINT;
typedef unsigned char BYTE;
typedef unsigned short WORD;
typedef unsigned long DWORD;
typedef long LONG;
typedef unsigned long ULONG;
typedef long long LONGLONG;
typedef unsigned long long ULONGLONG;
typedef uintptr_t ULONG_PTR;
typedef void* PVOID;
typedef void* HANDLE;
typedef void* HMODULE;
typedef void* HINSTANCE;
typedef void* HWND;
typedef void* LPVOID;
typedef const void* LPCVOID;
typedef int (*FARPROC)();
typedef wchar_t WCHAR;
typedef const wchar_t* LPCWSTR;
typedef const char* LPCSTR;
typedef DWORD (*LPTHREAD_START_ROUTINE)(LPVOID lpThreadParameter);

typedef struct _IMAGE_DOS_HEADER {
    WORD e_magic;
    WORD e_cblp;
    WORD e_cp;
    WORD e_crlc;
    WORD e_cparhdr;
    WORD e_minalloc;
    WORD e_maxalloc;
    WORD e_ss;
    WORD e_sp;
    WORD e_csum;
    WORD e_ip;
    WORD e_cs;
    WORD e_lfarlc;
    WORD e_ovno;
    WORD e_res[4];
    WORD e_oemid;
    WORD e_oeminfo;
    WORD e_res2[10];
    LONG e_lfanew;
} IMAGE_DOS_HEADER, *PIMAGE_DOS_HEADER;

typedef struct _IMAGE_FILE_HEADER {
    WORD Machine;
    WORD NumberOfSections;
    DWORD TimeDateStamp;
    DWORD PointerToSymbolTable;
    DWORD NumberOfSymbols;
    WORD SizeOfOptionalHeader;
    WORD Characteristics;
} IMAGE_FILE_HEADER, *PIMAGE_FILE_HEADER;

typedef struct _IMAGE_OPTIONAL_HEADER64 {
    WORD Magic;
    BYTE MajorLinkerVersion;
    BYTE MinorLinkerVersion;
    DWORD SizeOfCode;
    DWORD SizeOfInitializedData;
    DWORD SizeOfUninitializedData;
    DWORD AddressOfEntryPoint;
    DWORD BaseOfCode;
    ULONGLONG ImageBase;
    DWORD SectionAlignment;
    DWORD FileAlignment;
    WORD MajorOperatingSystemVersion;
    WORD MinorOperatingSystemVersion;
    WORD MajorImageVersion;
    WORD MinorImageVersion;
    WORD MajorSubsystemVersion;
    WORD MinorSubsystemVersion;
    DWORD Win32VersionValue;
    DWORD SizeOfImage;
    DWORD SizeOfHeaders;
    DWORD CheckSum;
    WORD Subsystem;
    WORD DllCharacteristics;
    ULONGLONG SizeOfStackReserve;
    ULONGLONG SizeOfStackCommit;
    ULONGLONG SizeOfHeapReserve;
    ULONGLONG SizeOfHeapCommit;
    DWORD LoaderFlags;
    DWORD NumberOfRvaAndSizes;
} IMAGE_OPTIONAL_HEADER64, *PIMAGE_OPTIONAL_HEADER64;

typedef struct _IMAGE_NT_HEADERS64 {
    DWORD Signature;
    IMAGE_FILE_HEADER FileHeader;
    IMAGE_OPTIONAL_HEADER64 OptionalHeader;
} IMAGE_NT_HEADERS64, *PIMAGE_NT_HEADERS64;

typedef struct _SYSTEM_INFO {
    DWORD dwNumberOfProcessors;
} SYSTEM_INFO, *LPSYSTEM_INFO;

inline void GetSystemInfo(LPSYSTEM_INFO info) {
    if (info) {
        long nprocs = sysconf(_SC_NPROCESSORS_ONLN);
        info->dwNumberOfProcessors = nprocs > 0 ? (DWORD)nprocs : 1;
    }
}

inline void* PosixThreadTrampoline(void* arg);

struct ThreadWrapper {
    LPTHREAD_START_ROUTINE routine;
    LPVOID param;
};

inline void* PosixThreadTrampoline(void* arg) {
    ThreadWrapper* w = (ThreadWrapper*)arg;
    LPTHREAD_START_ROUTINE r = w->routine;
    LPVOID p = w->param;
    delete w;
    DWORD res = r(p);
    return (void*)(intptr_t)res;
}

inline HANDLE CreateThread(void*, std::size_t, LPTHREAD_START_ROUTINE startRoutine, LPVOID param, DWORD, DWORD*) {
    pthread_t th;
    ThreadWrapper* w = new ThreadWrapper{startRoutine, param};
    if (pthread_create(&th, nullptr, PosixThreadTrampoline, w) != 0) {
        delete w;
        return nullptr;
    }
    return (HANDLE)(intptr_t)th;
}

inline DWORD WaitForSingleObject(HANDLE handle, DWORD) {
    if (!handle) return 0;
    pthread_t th = (pthread_t)(intptr_t)handle;
    pthread_join(th, nullptr);
    return 0;
}

inline DWORD WaitForMultipleObjects(DWORD nCount, const HANDLE* lpHandles, BOOL, DWORD) {
    for (DWORD i = 0; i < nCount; ++i) {
        if (lpHandles[i]) {
            pthread_join((pthread_t)(intptr_t)lpHandles[i], nullptr);
        }
    }
    return 0;
}

inline ULONGLONG GetTickCount64() {
    timespec ts{};
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return static_cast<ULONGLONG>(ts.tv_sec) * 1000ULL + static_cast<ULONGLONG>(ts.tv_nsec / 1000000ULL);
}

typedef union _LARGE_INTEGER {
    struct {
        DWORD LowPart;
        LONG HighPart;
    };
    struct {
        DWORD LowPart;
        LONG HighPart;
    } u;
    long long QuadPart;
} LARGE_INTEGER, *PLARGE_INTEGER;

typedef struct _WIN32_FIND_DATAW {
    DWORD dwFileAttributes;
    WCHAR cFileName[MAX_PATH];
} WIN32_FIND_DATAW, *PWIN32_FIND_DATAW, *LPWIN32_FIND_DATAW;

typedef struct _OVERLAPPED {
    ULONG_PTR Internal;
    ULONG_PTR InternalHigh;
    DWORD Offset;
    DWORD OffsetHigh;
    HANDLE hEvent;
} OVERLAPPED, *LPOVERLAPPED;

typedef struct _SRWLOCK {
    pthread_rwlock_t rwlock = PTHREAD_RWLOCK_INITIALIZER;
} SRWLOCK, *PSRWLOCK;

#define SRWLOCK_INIT {}

inline void InitializeSRWLock(PSRWLOCK lock) {
    pthread_rwlock_init(&lock->rwlock, nullptr);
}

inline void AcquireSRWLockExclusive(PSRWLOCK lock) {
    pthread_rwlock_wrlock(&lock->rwlock);
}

inline void ReleaseSRWLockExclusive(PSRWLOCK lock) {
    pthread_rwlock_unlock(&lock->rwlock);
}

inline void AcquireSRWLockShared(PSRWLOCK lock) {
    pthread_rwlock_rdlock(&lock->rwlock);
}

inline void ReleaseSRWLockShared(PSRWLOCK lock) {
    pthread_rwlock_unlock(&lock->rwlock);
}

inline void DisableThreadLibraryCalls(HMODULE) {}
inline HMODULE GetModuleHandleW(LPCWSTR) { return nullptr; }
inline FARPROC GetProcAddress(HMODULE, LPCSTR) { return nullptr; }

inline void SecureZeroMemory(void* ptr, std::size_t size) {
    explicit_bzero(ptr, size);
}

inline LONG InterlockedIncrement(LONG volatile* addend) {
    return __atomic_add_fetch(addend, 1, __ATOMIC_SEQ_CST);
}

inline LONG InterlockedDecrement(LONG volatile* addend) {
    return __atomic_sub_fetch(addend, 1, __ATOMIC_SEQ_CST);
}

inline DWORD GetCurrentProcessId() {
    return (DWORD)getpid();
}

inline DWORD GetCurrentThreadId() {
    return (DWORD)(uintptr_t)pthread_self();
}

inline DWORD GetLastError() {
    if (errno == EEXIST) return ERROR_ALREADY_EXISTS;
    if (errno == ENOENT) return ERROR_FILE_NOT_FOUND;
    return errno;
}

inline void SetLastError(DWORD err) {
    errno = err;
}

inline HANDLE CreateFileW(LPCWSTR path, DWORD access, DWORD, void*, DWORD creation, DWORD, HANDLE) {
    if (!path) return INVALID_HANDLE_VALUE;
    char mbs[MAX_PATH * 4] = {0};
    wcstombs(mbs, path, sizeof(mbs) - 1);

    int flags = 0;
    if ((access & GENERIC_READ) && (access & GENERIC_WRITE)) {
        flags |= O_RDWR;
    } else if (access & GENERIC_WRITE) {
        flags |= O_WRONLY;
    } else {
        flags |= O_RDONLY;
    }

    if (creation == CREATE_ALWAYS) {
        flags |= O_CREAT | O_TRUNC;
    } else if (creation == CREATE_NEW) {
        flags |= O_CREAT | O_EXCL;
    }

    int fd = open(mbs, flags, 0666);
    if (fd < 0) return INVALID_HANDLE_VALUE;
    return (HANDLE)(intptr_t)(fd + 1);
}

inline BOOL ReadFile(HANDLE file, LPVOID buffer, DWORD bytesToRead, DWORD* bytesRead, LPOVERLAPPED overlapped) {
    if (file == INVALID_HANDLE_VALUE || !file) return FALSE;
    int fd = (int)(intptr_t)file - 1;
    ssize_t res = 0;
    if (overlapped) {
        off_t off = (off_t)(((uint64_t)overlapped->OffsetHigh << 32) | overlapped->Offset);
        res = pread(fd, buffer, bytesToRead, off);
    } else {
        res = read(fd, buffer, bytesToRead);
    }
    if (res < 0) return FALSE;
    if (bytesRead) *bytesRead = (DWORD)res;
    return TRUE;
}

inline BOOL WriteFile(HANDLE file, LPCVOID buffer, DWORD bytesToWrite, DWORD* bytesWritten, void*) {
    if (file == INVALID_HANDLE_VALUE || !file) return FALSE;
    int fd = (int)(intptr_t)file - 1;
    ssize_t res = write(fd, buffer, bytesToWrite);
    if (res < 0) return FALSE;
    if (bytesWritten) *bytesWritten = (DWORD)res;
    return TRUE;
}

inline BOOL SetFilePointerEx(HANDLE file, LARGE_INTEGER distanceToMove, PLARGE_INTEGER newFilePointer, DWORD moveMethod) {
    if (file == INVALID_HANDLE_VALUE || !file) return FALSE;
    int fd = (int)(intptr_t)file - 1;
    int whence = SEEK_SET;
    if (moveMethod == FILE_CURRENT) whence = SEEK_CUR;
    else if (moveMethod == FILE_END) whence = SEEK_END;

    off_t res = lseek(fd, distanceToMove.QuadPart, whence);
    if (res == (off_t)-1) return FALSE;
    if (newFilePointer) newFilePointer->QuadPart = res;
    return TRUE;
}

inline BOOL GetFileSizeEx(HANDLE file, PLARGE_INTEGER fileSize) {
    if (file == INVALID_HANDLE_VALUE || !file || !fileSize) return FALSE;
    int fd = (int)(intptr_t)file - 1;
    struct stat st;
    if (fstat(fd, &st) != 0) return FALSE;
    fileSize->QuadPart = st.st_size;
    return TRUE;
}

inline BOOL FlushFileBuffers(HANDLE file) {
    if (file == INVALID_HANDLE_VALUE || !file) return FALSE;
    int fd = (int)(intptr_t)file - 1;
    return fsync(fd) == 0 ? TRUE : FALSE;
}

inline BOOL CloseHandle(HANDLE handle) {
    if (handle == INVALID_HANDLE_VALUE || !handle) return FALSE;
    int fd = (int)(intptr_t)handle - 1;
    return close(fd) == 0 ? TRUE : FALSE;
}

inline BOOL MoveFileExW(LPCWSTR existingName, LPCWSTR newName, DWORD) {
    if (!existingName || !newName) return FALSE;
    char from[MAX_PATH * 4] = {0};
    char to[MAX_PATH * 4] = {0};
    wcstombs(from, existingName, sizeof(from) - 1);
    wcstombs(to, newName, sizeof(to) - 1);
    return rename(from, to) == 0 ? TRUE : FALSE;
}

inline BOOL DeleteFileW(LPCWSTR path) {
    if (!path) return FALSE;
    char mbs[MAX_PATH * 4] = {0};
    wcstombs(mbs, path, sizeof(mbs) - 1);
    return unlink(mbs) == 0 ? TRUE : FALSE;
}

inline BOOL CreateDirectoryW(LPCWSTR path, void*) {
    if (!path) return FALSE;
    char mbs[MAX_PATH * 4] = {0};
    wcstombs(mbs, path, sizeof(mbs) - 1);
    if (mkdir(mbs, 0777) == 0) return TRUE;
    if (errno == EEXIST) {
        SetLastError(ERROR_ALREADY_EXISTS);
    }
    return FALSE;
}

inline DWORD GetFileAttributesW(LPCWSTR path) {
    if (!path) return INVALID_FILE_ATTRIBUTES;
    char mbs[MAX_PATH * 4] = {0};
    wcstombs(mbs, path, sizeof(mbs) - 1);
    struct stat st;
    if (stat(mbs, &st) != 0) return INVALID_FILE_ATTRIBUTES;
    return S_ISDIR(st.st_mode) ? FILE_ATTRIBUTE_DIRECTORY : FILE_ATTRIBUTE_NORMAL;
}

struct FindContext {
    DIR* dir;
    char pattern[MAX_PATH];
};

inline HANDLE FindFirstFileW(LPCWSTR path, LPWIN32_FIND_DATAW data) {
    if (!path || !data) return INVALID_HANDLE_VALUE;
    char mbs[MAX_PATH * 4] = {0};
    wcstombs(mbs, path, sizeof(mbs) - 1);

    // Extract directory
    char dirPath[MAX_PATH * 4] = {0};
    char* lastSlash = strrchr(mbs, '/');
    if (!lastSlash) lastSlash = strrchr(mbs, '\\');
    if (lastSlash) {
        size_t len = lastSlash - mbs;
        memcpy(dirPath, mbs, len);
        dirPath[len] = '\0';
    } else {
        strcpy(dirPath, ".");
    }

    DIR* dir = opendir(dirPath);
    if (!dir) {
        SetLastError(ERROR_PATH_NOT_FOUND);
        return INVALID_HANDLE_VALUE;
    }

    struct dirent* entry = readdir(dir);
    if (!entry) {
        closedir(dir);
        SetLastError(ERROR_FILE_NOT_FOUND);
        return INVALID_HANDLE_VALUE;
    }

    data->dwFileAttributes = (entry->d_type == DT_DIR) ? FILE_ATTRIBUTE_DIRECTORY : FILE_ATTRIBUTE_NORMAL;
    mbstowcs(data->cFileName, entry->d_name, MAX_PATH - 1);

    FindContext* ctx = new FindContext{dir, ""};
    return (HANDLE)ctx;
}

inline BOOL FindNextFileW(HANDLE handle, LPWIN32_FIND_DATAW data) {
    if (handle == INVALID_HANDLE_VALUE || !handle || !data) return FALSE;
    FindContext* ctx = (FindContext*)handle;
    struct dirent* entry = readdir(ctx->dir);
    if (!entry) {
        SetLastError(ERROR_NO_MORE_FILES);
        return FALSE;
    }
    data->dwFileAttributes = (entry->d_type == DT_DIR) ? FILE_ATTRIBUTE_DIRECTORY : FILE_ATTRIBUTE_NORMAL;
    mbstowcs(data->cFileName, entry->d_name, MAX_PATH - 1);
    return TRUE;
}

inline BOOL FindClose(HANDLE handle) {
    if (handle == INVALID_HANDLE_VALUE || !handle) return FALSE;
    FindContext* ctx = (FindContext*)handle;
    if (ctx->dir) closedir(ctx->dir);
    delete ctx;
    return TRUE;
}

#endif
