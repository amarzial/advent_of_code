#define FT_OPEN_HEADERS
#include "libft.h"

int	istriangle(int *sizes)
{
	int	m;

	m = ft_max(sizes[0], ft_max(sizes[1], sizes[2]));
	if (((sizes[0] + sizes[1] + sizes[2]) - m) > m)
		return (1);
	return (0);
}

int main(int argc, const char *argv[])
{
	int		fd;
	char	*line;
	char	**vals;
	int		len[3];
	int		count = 0;

	fd = open("input.txt", O_RDONLY);
	while (ft_get_line(fd, &line) == 1)
	{
		vals = ft_strsplit(line, ' ');
		len[0] = ft_atoi(vals[0]);
		len[1] = ft_atoi(vals[1]);
		len[2] = ft_atoi(vals[2]);
		if (istriangle(len))
			count++;
		free(line);
	}
	ft_putnbr(count);
	return 0;
}
