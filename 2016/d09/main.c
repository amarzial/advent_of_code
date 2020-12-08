#define FT_OPEN_HEADERS
#include "libft.h"
#define BUFF_SIZE 100000

int	decode(char **line)
{
	int len;
	int	times;

	len = ft_atoi((*line + 1));
	*line = ft_strchr(*line, 'x');
	times = ft_atoi((*line + 1));
	*line = ft_strchr(*line, ')') + len;
	return (len * times);
}

int	expand(char *line)
{
	int	cnt;

	cnt = 0;
	while (*line)
	{
		if (*line != '(')
		{
			if (*line != ' ' && *line != '\n')
				cnt++;
		}
		else
			cnt += decode(&line);
		line++;
	}
	return (cnt);
}

int main(int argc, const char *argv[])
{
	int		fd;
	char	buffer[BUFF_SIZE];
	int		cnt;

	if (!(argc == 2 && (fd = open(argv[1], O_RDONLY)) >= 0))
		return (1);
	cnt = read(fd, buffer, BUFF_SIZE);
	buffer[cnt] = '\0';
	cnt = expand(buffer);
	ft_printf("Length: %d", cnt);
	return 0;
}
