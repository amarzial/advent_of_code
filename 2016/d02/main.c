#include "libft.h"
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

int	getnum(char *line)
{
	static int	num = 5;
	char	tmp;

	while (*line)
	{
		tmp = *line++;
		if (tmp == 'U'&& (num > 3))
			num -= 3;
		else if (tmp == 'D' && num < 7)
			num += 3;
		else if (tmp == 'L' && (num % 3) != 1)
			num -= 1;
		else if (tmp == 'R' && (num % 3) != 0)
			num += 1;
	}
	return (num);
}

int main(int argc, const char *argv[])
{
	int		fd;
	char	*line;
	int		num;

	fd = open("input.txt", O_RDONLY);
	while (ft_get_line(fd, &line) > 0)
	{
		ft_putnbr(getnum(line));
		free(line);
	}

	return 0;
}
