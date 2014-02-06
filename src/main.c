#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
  char *path = "PATH";
  char *path_value = getenv(path);

#if _WIN32
  const char *path_sep = ";";
#else
  const char *path_sep = ":";
#endif

  char *path_item;
  path_item = strtok(path_value, path_sep);
  while (path_item != NULL) {
	printf("%s\n", path_item);
	path_item = strtok(NULL, path_sep);
  }

  return EXIT_SUCCESS;
}
