#define FT_OPEN_HEADERS
#include "libft.h"

void shifter(char *str, int val)
{
	while (*str)
	{
		ft_putchar(((*str - 'a') + val) % 26 + 'a');
		str++;
	}
}

void decode(char **tab, int size, int shift)
{
	int n;

	n = 0;
	while (n < size - 1)
	{
		shifter(tab[n], shift % 26);
		ft_putchar(' ');
		n++;
	}
	ft_putnbr(shift);
	ft_putchar('\n');
}

int roomval(char *line)
{
	int cnt;
	char **tab;
	int len;
	int out;
	int cur;
	int tmp;

	tab = ft_strsplit(line, '-');
	len = 0;
	while (tab[len])
		len++;
	cur = 4;
	while (cur < 9)
	{
		tmp = ft_strcnt(line, tab[len - 1][cur]);
		if (cur != 4)
		{
			if (tmp == 1 || tab[len - 1][cur] < tab[len - 1][cur - 1] && tmp == cnt || tmp > cnt)
			{
				tmp = -1;
				break ;
			}
		}
		cnt = tmp;
		cur++;
	}
	out = ft_atoi(tab[len - 1]);
	if (tmp >= 0)
		decode(tab, len, out);
	while (len--)
		free(tab[len]);
	return (tmp == -1 ? 0 : out);
}

int main(int argc, const char *argv[])
{
	int fd;
	char *line;
	int out = 0;

	fd = open("input.txt", O_RDONLY);

	while (ft_get_line(fd, &line) > 0)
	{
		out += roomval(line);
		free(line);
	}
	ft_putnbr(out);
	return 0;
}
