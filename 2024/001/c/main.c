#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <stdint.h>
#include <stdlib.h>
#include <string.h>

#define MIN(a, b) a < b ? a : b

int read_file_to_string(const int fd, char **outstr, ssize_t *len) {
  #define _READ_SIZE (1 << 10)

  int buf_size = 0;
  char *buf = NULL, *cbuf = NULL;
  ssize_t nb_read;
  ssize_t total_read = 0;

  for (;;) {
    buf_size += _READ_SIZE;
    buf = realloc(buf, buf_size);
    cbuf = buf + buf_size - _READ_SIZE;

    nb_read = read(fd, cbuf, _READ_SIZE);
    if      (nb_read == 0) { break; } // EOF
    else if (nb_read < 0) {
      fprintf(stderr, "Error occurred during read.\n");
      free(buf); buf = NULL;
      return -1;
    }
    total_read += nb_read;
  }
  *outstr = buf;
  *len = total_read;
  return 0;
}

void parse_int(const char *const buf, const int buflen, int32_t *out) {
  int32_t parsed = 0;
  int idx = 0;
  while (idx < buflen) {
    parsed *= 10;
    const uint8_t c = (uint8_t) buf[idx];
    parsed += c - 48; // 48 = ascii num 0
    ++idx;
  }
  *out = parsed;
}

void parse_left_right_numbers(const char *const line, const int linelen, int32_t *l, int32_t *r) {
  int sep = 0;
  while (sep < linelen && line[sep] != ' ') ++sep;
  parse_int(line, sep, l);
  parse_int(line+sep+3, linelen-sep-4, r);
}

// Find the index of `c` in `buf`, starting from `start_idx`.
int index_of_char(const char *const buf, const int bufsize, const int start_idx, char c) {
  int idx = start_idx;
  while (idx < bufsize && buf[idx++] != c) {}
  return idx;
}

void parse_numbers_from_file(const int fd, int32_t **left, int32_t **right, int32_t *size) {
  char *contents = NULL;
  ssize_t len = 0;
  if (read_file_to_string(fd, &contents, &len) < 0 || len == 0) { return; }

  int32_t *left_arr = NULL, *right_arr = NULL;
  int32_t arr_size = 0, arr_alloc_size = 512;
  left_arr = malloc(sizeof(int32_t) * arr_alloc_size);
  right_arr = malloc(sizeof(int32_t) * arr_alloc_size);

  // parse the contents by newline
  int prev_idx = 0, idx;
  int32_t left_num, right_num;
  while ((idx = index_of_char(contents, len, prev_idx, '\n'))) {
    if (idx == prev_idx) break;
    parse_left_right_numbers(contents+prev_idx, idx-prev_idx, &left_num, &right_num);
    ++arr_size;
    if (arr_size * sizeof(int32_t) > arr_alloc_size) {
      arr_alloc_size += 512 * sizeof(int32_t);
      left_arr = realloc(left_arr, arr_alloc_size);
      right_arr = realloc(right_arr, arr_alloc_size);
    }
    left_arr[arr_size-1] = left_num;
    right_arr[arr_size-1] = right_num;
    prev_idx = idx;
  }
  *left = left_arr;
  *right = right_arr;
  *size = arr_size;
  free(contents);
}

int i32_comp(const void* a, const void *b) {
  const int32_t ia = *((int32_t*) a);
  const int32_t ib = *((int32_t*) b);
  return ia > ib;
}

void compute(char *input_file) {
  const int fd = open(input_file, 0, 0);
  if (fd < 0) {
    fprintf(stderr, "Failed to open input file: %s\n", input_file);
  }

  // left and right will contain the numbers on the left and right
  // for each line of the input file. nb_entries is the size of
  // these arrays.
  int32_t *left = NULL, *right = NULL;
  int32_t nb_entries = 0;
  parse_numbers_from_file(fd, &left, &right, &nb_entries);
  qsort(left, nb_entries, sizeof(int32_t), i32_comp);
  qsort(right, nb_entries, sizeof(int32_t), i32_comp);

  int64_t total_diffs = 0;
  for (int i = 0; i < nb_entries; ++i) {
    const int d = abs(left[i] - right[i]);
    total_diffs += d;
  }

  free(left);
  free(right);
  close(fd);
  printf("Answer: %ld\n", total_diffs);
}

int main() {
  compute("2024/001/input.txt");
  return 0;
}