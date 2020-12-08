#define FT_OPEN_HEADERS
#include "libft.h"

int	check(char *str)
{
	int		valid;
	char	*start;

	valid = 0;
	start = str;
	while (*(str + 3) != 0)
	{
		if (*(str + 3) == '[')
		{
			while (*(str + 3) != ']')
			{
				if (str[0] == str[3] && str[1] == str[2] && str[0] != str[1])
					return (0);
				str++;
			}
			str += 4;
		}
		if (*str == '[' && str++)
			continue ;
		if (!*str)
			break ;
		if (str[0] == str[3] && str[1] == str[2] && str[0] != str[1])
			valid = 1;
		str++;
	}
	if (valid)
	{
		ft_putendl(start);
		return (1);
	}
	return (0);
}

int	main(int argc, const char *argv[])
{
	int		fd;
	char	*line;
	int		res;

	res = 0;
	fd = open("input.txt", O_RDONLY);
	while (ft_get_line(fd, &line) > 0)
	{
		res += check(line);
		free(line);
	}
	ft_putnbr(res);
	return 0;
}
