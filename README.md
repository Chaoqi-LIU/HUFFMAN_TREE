# Huffman Encoding/Decoding

This project is inspired by [lab_huffman](https://courses.engr.illinois.edu/cs225/fa2022/labs/huffman/) of CS225(2022fall) @ UIUC

Team: \
&emsp;&emsp; -- Chaoqi LIU (chaoqil2@illinois.edu) \
&emsp;&emsp; -- Jiahui LIN (jiahui9@illinois.edu)
  
---

## Background Information
In 1951, while enrolled in an Information Theory class at MIT, David A. Huffman and his classmates were given a choice by professor Robert M. Fano: they could either take the final exam or find the most efficient binary code. Huffman chose the less traveled path, and the rest, as they say, is history.

The Huffman encoding algorithm is a fundamental data compression algorithm. Data compression is a powerful tool that allows a given set of information to be represented in less space, allowing for more efficient data transfer. JPG (lossy) and PNG image formats use various types of compression (lossless). It is also used to compress multiple files in ZIP files. Communication Networks, which deal with transferring large amounts of data, and Computer Security, which deals with data encoding for a layer of privacy, both use the concept of data encoding.

[Huffman coding](https://en.wikipedia.org/wiki/Huffman_coding)


## Sample
<p>
                                                                                                ______________________________ 75 _____________________________                                                                                                  
                                                                 ______________________________/                                                               \______________________________                                                                   
                                                ______________ 32 _____________                                                                                                ______________ 43 _____________                                                   
                                 ______________/                               \______________                                                                  ______________/                               \______________                                    
                        ______ 15 _____                                                ______ 17 _____                                                 ______ 19 _____                                                ______ 24 _____                            
                 ______/               \______                                  ______/               \______                                   ______/               \______                                  ______/               \______                     
            __ 7 __                          a:8                           __ 8 __                          _:9                            __ 9 __                        __ 10 _                         __ 12 _                        __ 12 _                 
         __/       \__                                                  __/       \__                                                   __/       \__                  __/       \__                   __/       \__                  __/       \__              
      r:3             4                                              e:4            t:4                                               4             m:5             o:5            i:5               6              6               6             n:6            
                    /   \                                                                                                           /   \                                                          /   \          /   \           /   \                          
                  2      2                                                                                                       u:2    c:2                                                     l:3    s:3     f:3    h:3      d:3    g:3                        
                 / \    / \                                                                                                                                                                                                                                      
               T:1.:1 p:1H:1                                                                                                                                                                                                                                     

</p>
