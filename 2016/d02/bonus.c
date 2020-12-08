#include "libft.h"
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>
#define CHARSET "123456789ABCD"

int	getnum(char *line)
{
	static int	num = 5;
	char	tmp;

	while (*line)
	{
		tmp = *line++;
		if (tmp == 'U'&& (num == 3 || (num >= 6 && num <= 8) || (num >= 10)))
		{
			num += (num == 3 || num == 13) ? -2 : 0;
			num += ((num >= 6 && num <= 8) || (num >= 10 && num <= 12))? -4 : 0;
		}
		else if (tmp == 'D' && (num == 11 || (num >=6 && num <= 8) || num <= 4))
		{
			num += (num == 1 || num == 11) ? +2 : 0;
			num += ((num >= 6 && num <= 8) || (num >= 2 && num <= 4))? +4 : 0;
		}
		else if (tmp == 'L' && (num != 1 && num !=2 && num != 5 && num != 10 && num != 13))
			num -= 1;
		else if (tmp == 'R' && (num != 1 && num !=4 && num != 9 && num != 12 && num != 13))
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
		ft_putchar(CHARSET[getnum(line) - 1]);
		free(line);
	}

	return 0;
}
