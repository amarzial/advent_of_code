#define FT_OPEN_HEADERS
#include "libft.h"

int	istriangle(int *val)
{
	int	m;

	m = ft_max(val[0], ft_max(val[1], val[2]));
	if (((val[0] + val[1] + val[2]) - m) > m)
		return (1);
	return (0);
}

int main(int argc, const char *argv[])
{
	int		fd;
	char	*line[3];
	char	**vals[3];
	int		len[3];
	int		count = 0;
	int		cur;

	fd = open("input.txt", O_RDONLY);
	while (ft_get_line(fd, line) == 1)
	{
		ft_get_line(fd, line + 1);
		ft_get_line(fd, line + 2);
		vals[0] = ft_strsplit(line[0], ' ');
		vals[1] = ft_strsplit(line[1], ' ');
		vals[2] = ft_strsplit(line[2], ' ');
		cur = 0;
		while (cur < 3)
		{
			len[0] = ft_atoi(vals[0][cur]);
			len[1] = ft_atoi(vals[1][cur]);
			len[2] = ft_atoi(vals[2][cur++]);
			if (istriangle(len))
				count++;
		}
		free(line[0]);
		free(line[1]);
		free(line[2]);
	}
	ft_putnbr(count);
	return 0;
}
