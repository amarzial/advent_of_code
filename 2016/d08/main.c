#define FT_OPEN_HEADERS
#include "libft.h"
#define WIDTH 50
#define HEIGHT 6

static void		print(char grid[][WIDTH])
{
	int y;
	int x;
	int cnt;

	y = 0;
	cnt = 0;
	while (y < HEIGHT)
	{
		x = 0;
		while (x < WIDTH)
			cnt += (grid[y][x++] == '1') ? 1 : 0;
		ft_printf("%.*s|\n", WIDTH,  grid[y++]);
	}
	ft_printf("Lit: %d\n", cnt);
}

static void		rect(char grid[][WIDTH], char *line)
{
	int	x, y;
	int	tmp;

	x = ft_atoi(line);
	y = ft_atoi(ft_strchr(line, 'x') + 1);
	tmp = x;
	while (y-- && (x = tmp))
		while (x--)
			grid[y][x] = '1';
}

static void		rotate(char grid[][WIDTH], char *line)
{
	int	x,y;
	if (ft_strstr(line, "row"))
	{
		char tmp[WIDTH];
		y = ft_atoi(ft_strchr(line, '=') + 1);
		x = ft_atoi(ft_strstr(line, "by") + 3) % WIDTH;
		ft_memcpy(tmp, grid[y] + (WIDTH - x), x);
		ft_memmove(grid[y] + x, grid[y], WIDTH - x);
		ft_memcpy(grid[y], tmp, x);
	}
	else if (ft_strstr(line, "column"))
	{
		x = ft_atoi(ft_strchr(line, '=') + 1);
		y = ft_atoi(ft_strstr(line, "by") + 3) % HEIGHT;
		while (y--)
		{
			char tmp;
			int cnt;

			tmp = grid[HEIGHT - 1][x];
			cnt = HEIGHT - 1;
			while (cnt--)
				grid[cnt + 1][x] = grid[cnt][x];
			grid[0][x] = tmp;
		}
	}
}

int				main(int argc, const char *argv[])
{
	char		grid[HEIGHT][WIDTH];
	int		cur = 0;
	int		fd;
	char	*line;

	while (cur < HEIGHT)
	{
		ft_memset(grid[cur++], ' ', WIDTH);
	}
	fd = open("input.txt", O_RDONLY);
	while (ft_get_line(fd, &line) > 0)
	{
		if (ft_strstr(line, "rect"))
			rect(grid, line + 5);
		else if (ft_strstr(line, "rotate"))
			rotate(grid, line + 7);
		free(line);
	}
	print(grid);
	return 0;
}
