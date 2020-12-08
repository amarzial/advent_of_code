#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#define DIRECTIONS 4

typedef struct	s_point
{
	int x;
	int y;
}				t_point;

int main(int argc, const char *argv[])
{
	t_point	target;
	t_point	orint[DIRECTIONS] = {{1, 0}, {0, 1}, {-1, 0}, {0, -1}};
	int		dir = 0;
	char	buffer[5000];
	int		fd;
	int		val;
	int		idx = 0;

	target.x = 0;
	target.y = 0;
	memset(buffer, 0, 5000);
	if ((fd = open("input.txt", O_RDONLY)) > 0)
	{
		read(fd, buffer, 5000);
	}
	while (buffer[idx])
	{
		if (buffer[idx] == 'R')
			dir++;
		else if (buffer[idx] == 'L')
			dir--;
		idx++;
		if (dir >= DIRECTIONS)
			dir -= DIRECTIONS;
		else if (dir < 0)
			dir += DIRECTIONS;
		val = atoi(buffer + idx);
		target.x += val * orint[dir].y;
		target.y += val * orint[dir].x;
	printf("x: %d, y: %d\n", target.x, target.y);
		while (buffer[idx] != ' ' && buffer[idx] != '\0')
			idx++;
		idx++;
	}
	printf("distance = %d\n", abs(target.x) + abs(target.y));
	return 0;
}
