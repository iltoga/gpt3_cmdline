
# README.md
This code provides an interface for users to generate text and images using OpenAI's GPT-3 language model. Users can interact with the program via the terminal to input questions and receive generated text or images as output.

## How to Run
1. Clone this repository on your computer.
2. Obtain an API key from [OpenAI](https://beta.openai.com/signup/).
   To do that, once signed in to openai, go to https://platform.openai.com/account/api-keys and generate one
3. Set the `OPENAI_KEY` environment variable to your API key.
4. In the terminal, navigate to the project directory and run the following commands:

```bash
# Set the OPENAI_KEY environment variable to your API key
export OPENAI_KEY=your-api-key-here
# Run the Python script that uses the OpenAI API
cargo run
```

## Code Overview
The code consists of several structs that define the format of the input and output messages to the OpenAI API. The `TextGenerationRequest` struct contains fields for the model name, prompt, and various generation parameters, while the `TextGenerationResponse` struct contains fields for the generated text and usage statistics. Similarly, the `ImageGenerationRequest` and `ImageGenerationResponse` structs define the input and output for image generation.

The `send_request` function sends the input request to the OpenAI API and returns a result of either the output response or an error message. The `generate_text` and `generate_image` functions use this `send_request` function to send text and image generation requests to the API, respectively.

In the `main` function, the user is prompted to input a question and select the desired output format (text or image). Depending on the output format selected, the user is prompted for additional input (such as the model name for text generation). Finally, the appropriate `generate` function is called with the user input, and the output is printed to the console.