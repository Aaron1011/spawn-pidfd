

#include <sys/socket.h>

//#define ONE_FD_BUF CMSG_SPACE(sizeof int[1])

static int fd_buf[1];


#define SIZEOF_INT_BUF sizeof fd_buf
#define ONE_FD_BUF CMSG_SPACE(sizeof fd_buf)


const unsigned long int FD_BUF_SIZE = ONE_FD_BUF; 
