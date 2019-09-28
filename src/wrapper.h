#include <sys/socket.h>

static int fd_buf[1];
const unsigned long int ONE_FD_BUF_SIZE = CMSG_SPACE(sizeof fd_buf);
