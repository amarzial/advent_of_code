#include <iostream>
#include <stdio.h>
#include <openssl/md5.h>

int main()
{
	unsigned char hash[MD5_DIGEST_LENGTH + 1];
	const unsigned char *salt = (unsigned char*)"ciao";

	MD5(salt, 4, hash);
	hash[MD5_DIGEST_LENGTH] = '\0';
	for (int i = 0; i < MD5_DIGEST_LENGTH; i++)
		printf("%.2x", hash[i]);
	return 0;
}