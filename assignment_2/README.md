# Webserver on Rust  
---------------------------------
## How I build it ? 

This inital framework was taken inspiration from chapter_20.1 of Rust book, **Building a Single-Threaded Webserver** and 
then I added upon the code to take into account various errors .

##### The errors I have taken into accounted are : 
* ERROR 400 (Bad Request): It refers to bad-request error . It is trigerred when malformed requests are sent to webservers .
  To respond to this I check the method and version in request line .
 ![image](https://github.com/fidgetaryan445/Rustlings_230217/assets/148867576/a9310023-b92a-4077-8747-e7a1f2968f32)
* ERROR 403 (Forbidden): It means the requested source is forbidden . For eg: If the requested page doesn't have read permission then
it would show error 403 . I used ErrorKind::PermissionDenied to takle this error .
![image](https://github.com/fidgetaryan445/Rustlings_230217/assets/148867576/0dbcb788-053e-450f-9ac9-6eabb0d5b3a8)
* Error 404 (Not Found) : It means the requested page doesn't exist . So in my webserver I am only responding to requests of 2 pages .
For my read page if it somehow is deleted then my webpage shows error 404 and for all other pages that are requested I show error 404.
![image](https://github.com/fidgetaryan445/Rustlings_230217/assets/148867576/7de07c2d-2745-4681-8230-d6d7fdb0e212)
* Error 405 (Method Not Allowed): It is exectued when a request is made with a disallowed method  . As my sevver only entertains GET request all other
valid methods triger this error  .
![image](https://github.com/fidgetaryan445/Rustlings_230217/assets/148867576/c3c3a0c5-7bf0-4f5c-8b9d-7b6b2a7bbd6e)
* Error 500 (Internal Server Error) : I have handled it only in the case where my `/read` couldn't be displayed even though it exists
* and has read permissions but still couldn't load . This could occur when any other application is actually using the file and has locked it
* so the server can't read it .

