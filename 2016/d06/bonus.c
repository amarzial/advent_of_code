#define FT_OPEN_HEADERS
#include "libft.h"

int main(int argc, const char *argv[])
{
	int		map[8][26];
	int		cur;
	char	*line;
	int		fd;
	int		max;

	cur = 0;
	while (cur < 8)
		ft_bzero(map[cur++], sizeof(int) * 26);
	fd = open("input.txt", O_RDONLY);
	while (ft_get_line(fd, &line) > 0)
	{
		cur = 0;
		while (cur < 8)
		{
			(map[cur][line[cur] - 'a'])++;
			cur++;
		}
		free(line);
	}
	close(fd);
	cur = 0;
	while (cur < 8)
	{
		fd = 0;
		max = 0;
		while (fd < 26)
		{
			if (map[cur][fd] < map[cur][max])
				max = fd;
			fd++;
		}
		cur++;
		ft_putchar(max + 'a');
	}
	ft_putchar('\n');
	return 0;
}
