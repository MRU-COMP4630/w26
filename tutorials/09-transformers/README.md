# Tutorial 10: Applied Transformers

We don't really have the computational resources to train our own transformer models, but we can play around with pre-trained models.

The [HuggingFace Transformers](https://huggingface.co/docs/transformers/en/index) API is an easy way to download a pre-trained model and either use as-is, or fine-tune for your application. I'll provide a few suggestions, but the official [tutorial](https://huggingface.co/docs/transformers/pipeline_tutorial) has a lot more info.

## Using pretrained transformer models
If you're working on Kaggle or Colab, the transformers library may already be installed; otherwise, you'll need to `pip install transformers`.

1. Pick a task and a model from the [model list]() - I recommend sticking to something on the small side for your own sanity. Text generation is fun to play with so I'm going to start there, e.g.:
    ```python
    from transformers import pipeline

    pipe = pipeline("text-generation", model="deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B")
    ```
    > [!TIP]
    > If you're on a GPU-enabled platform like Kaggle, you can move the model to GPU as follows:
    > ```python
    > from accelerate import Accelerator
    >
    > device = Accelerator().device
    > 
    > pipe = pipeline("text-generation", model="deepseek-ai/DeepSeek-R1-Distill-Qwen-1.5B", device=device)
    > ```

2. Test it out! The simplest way to interact with a text generation model is to pass it a string:
   ```python
   pipe("What is the airspeed velocity of an unladen swallow?")
   ```

3. Add context. Some of the models (like the Deepseek example) allow additional chat context, e.g.:
    ```python
    messages = [
        {"role": "user", "content": "What is the airspeed velocity of an unladen swallow?"},
    ]
    pipe(messages)
    ```
    How does this affect the value that is returned?

4. You can inspect and modify the configuration of the pipeline with the `pipe.generation_config` object. Try changing the `temperature` parameter and see how it impacts the results, e.g.:
   ```python
   pipe.generation_config.temperature = 1.5
   ```
   I'd also recommend reducing the `max_new_tokens` so that it runs faster and doesn't ramble quite so much!

5. To make a real interactive chatbot, create a sentinel loop that:
    1. prompts for input
    2. passes the input to the pipeline
    3. prints out the response
    4. concatenates the response with the previous messages
    5. prompts for input again

There's lots more to play around with in this library, but that's probably plenty for the tutorial time period.