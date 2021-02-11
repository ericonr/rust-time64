#include <time.h>
#include <stdint.h>

static struct timespec t;

void receive_time64(int64_t sec, int32_t nsec)
{
	t.tv_sec = sec;
	t.tv_nsec = nsec;
}

const struct timespec *ret_p(void)
{
	return &t;
}
