#include <stdio.h>
#include <stdlib.h>

char *read_file_content(const char *filename) {
  FILE *file = fopen(filename, "rb");
  if (file == NULL) {
    perror("Error opening file");
    return NULL;
  }

  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);
  rewind(file);

  char *buffer = (char *)malloc(file_size + 1);
  if (buffer == NULL) {
    perror("Error allocating memory");
    fclose(file);
    return NULL;
  }

  size_t read_size = fread(buffer, 1, file_size, file);
  if (read_size != file_size) {
    perror("Error reading file");
    free(buffer);
    fclose(file);
    return NULL;
  }

  buffer[file_size] = '\0';

  fclose(file);
  return buffer;
}

#include "xtql_c.h"

int main(int argc, char **argv) {
  const char *content = NULL;
  const char *result = NULL;
  if (argc != 2) {
    fprintf(stderr, "usage: a.out path-to-file\n");
    exit(1);
  }

  content = read_file_content(argv[1]);
  if (content == NULL) {
    fprintf(stderr, "Failed to read file\n");
    return 1;
  }

  result = xtql_json_c(content);
  printf("%s\n", result);

  free_rust_string(result);
  free((void *)content);

  return 0;
}
