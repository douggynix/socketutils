# socketutils
A rust utility program developped by me to list list all tcp sockets used on a linux machine in a parsable manner with relevant fields:
![image](https://github.com/douggynix/socketutils/assets/11457752/5f28276f-59bd-4c01-b8ec-0c457b27aeee)

Here is the display on a Github Action Runner container after build
![image](https://github.com/douggynix/socketutils/assets/11457752/df038474-7c2c-46c8-9b65-910854bcceb9)

socketutils vs netstat perf (4ms difference). That 4ms is due to the fact that socketutils outputs more data to the screen. 
With similar data ouput size, the runtime execution would be the same.
![image](https://github.com/douggynix/socketutils/assets/11457752/123e58f8-fa27-4d2e-8511-2bd01cadbe69)

![image](https://github.com/douggynix/socketutils/assets/11457752/c53b0abe-de2e-4c3c-be84-8f024d22de46)

![image](https://github.com/douggynix/socketutils/assets/11457752/c960d741-684f-42db-90da-3f5f7203be2c)


