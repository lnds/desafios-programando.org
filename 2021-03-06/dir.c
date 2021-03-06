/* dir.c */
#include <stdlib.h>
#include <stdio.h>
#include <dirent.h>
#include <limits.h>
#include <unistd.h>

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

    cwd = getcwd(buff, max_path);
    printf("directorio: %s\n", cwd);
    dir = opendir(cwd);
    while ((dp = readdir(dir)) != NULL) {
        if (dp->d_name[0] == '.')
            continue;
        printf("%s, inode: %llu\n", dp->d_name, dp->d_ino);
    }
    closedir(dir);
    return 0;
}