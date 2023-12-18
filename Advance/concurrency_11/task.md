# Task: Implement a Multi-threaded Word Count

**Objective:**
Write a Rust program that uses multiple threads to count the occurrences of words in a set of texts. Your program should split the text into words, send each word to a worker thread for processing, and then aggregate the results.

**Requirements:**

1. **Data Preparation:**

   - Define a vector of strings, where each string represents a separate text (e.g., sentences or paragraphs).
   - Example: `let texts = vec!["text one", "text two", "text three", ...];`

2. **Thread Creation and Management:**

   - Use `thread::spawn` to create multiple worker threads. Each thread should be responsible for processing part of the data.
   - Use channels (mpsc) to send individual words to the worker threads for processing.

3. **Word Count Logic:**

   - Each worker thread should maintain a local count of the words it processes.
   - After processing, each thread should send its local count back to the main thread.

4. **Aggregating Results:**

   - The main thread should aggregate the counts from all worker threads to produce a final count of word occurrences.
   - Print the aggregated results in the main thread.

5. **Handling Errors:**

   - Ensure that your program gracefully handles any potential errors, such as issues with thread spawning or message passing.

6. **Optional Challenges:**
   - Implement functionality to exclude common stop words from the count.
   - Enhance the program to process the texts in parallel, rather than sequentially feeding words to the threads.

**Example Output:**

```
Word 'hello' occurred 3 times.
Word 'world' occurred 2 times.

```
