/* dir3.c */
#include <stdlib.h>
#include <stdio.h>
#include <dirent.h>
#include <fcntl.h>
#include <stdint.h>
#include <limits.h>
#include <unistd.h>
#include <sys/stat.h>

long get_path_max() {
    long path_max = pathconf(".", _PC_PATH_MAX);
    if (path_max == -1) {
        return 1024;
    }
    return path_max;
}

int main(int argc, char* argv[]) {
    char* cwd;
    DIR* dir;
    struct dirent *dp;
    long max_path = get_path_max();
    char* buff = (char*) malloc(max_path);
    int dfd, ffd;
    struct stat statbuf;

    cwd = getcwd(buff, max_path);
    printf("directorio: %s\n", cwd);
    dfd = open(cwd, O_RDONLY);
    if ((dir = fdopendir(dfd)) == NULL) {
        fprintf(stderr, "Cannot open ./tmp directory\n");
        exit(1);
    }
    while ((dp = readdir(dir)) != NULL) {
        if (dp->d_name[0] == '.')
            continue;
        if ((ffd = openat(dfd, dp->d_name, O_RDONLY)) == -1) {
            perror(dp->d_name);
            continue;
        }
        if (fstat(ffd, &statbuf) == 0) {
            printf("%s, inode: %llu, bytes: %lld, owner: %d, group: %d, mode: %d|%d|%d\n", 
            dp->d_name, dp->d_ino, statbuf.st_size, statbuf.st_uid, statbuf.st_gid, 
            (statbuf.st_mode >>  6) & 0x07,
            (statbuf.st_mode >> 3) & 0x07,
            statbuf.st_mode & 0x07
            
            );
        }
        close(ffd);
    }
    closedir(dir);
    return 0;
}