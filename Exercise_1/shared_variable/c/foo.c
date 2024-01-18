// Compile with `gcc foo.c -Wall -std=gnu99 -lpthread`, or use the makefile
// The executable will be named `foo` if you use the makefile, or `a.out` if you use gcc directly

#include <pthread.h>
#include <stdio.h>

int i = 0;
pthread_mutex_t lock;

// Note the return type: void*
void* incrementingThreadFunction(){
    for(int j = 0; j < 1000000; j++) {
        pthread_mutex_lock(&lock);
        i += 1;
        pthread_mutex_unlock(&lock);
    }
    return NULL;
}

void* decrementingThreadFunction(){
    for(int j = 0; j < 999900; j++) {
        pthread_mutex_lock(&lock);
        i -= 1;
        pthread_mutex_unlock(&lock);
    }
    return NULL;
}


int main(){

    // Spawn threads
    pthread_t thread1;
    pthread_t thread2;
    pthread_create(&thread1, NULL, incrementingThreadFunction, "Thread 1");
    pthread_create(&thread2, NULL, decrementingThreadFunction, "Thread 2");

    // Initialize mutex
    pthread_mutex_init(&lock, NULL);

    // Wait for threads to finish
    pthread_join(thread1, NULL);
    pthread_join(thread2, NULL);
    
    printf("The magic number is: %d\n", i);
    return 0;
}
