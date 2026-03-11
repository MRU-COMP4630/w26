# Assignment 3: Classification of Text Data
Due March ~~20~~ 27, 2026

You may work in teams up to 3. Click [here](https://classroom.github.com/a/RrRSlXrr) to create your team on GitHub Classroom.

## Overview
The purpose of this assignment is to further your hands-on experience in neural networks with a new type of data: text! 

## Dataset and task
Download the pre-split "Reddit jokes" csv [here](https://drive.google.com/file/d/1pLjqkGdGotkVg8Wdi4WDPzt1PiIRmYk2/view?usp=sharing) (200 MB). You will still need to do a validation split, but I've reserved a final test set as with the previous assignments.

The original dataset is [1 million reddit jokes](https://huggingface.co/datasets/SocialGrep/one-million-reddit-jokes), but I've modified it a bit. I added a threshold to the `score` value to classify each joke as `is_funny = True` or `is_funny=False`, and I've also removed some redundant columns. I left the score value in there so you can see just how "funny" people thought a given joke was, but **the score cannot be used as a predictor**.

Your task is to predict whether or not a joke is funny. You may use any of the information in the dataset **except for the score column** to predict the `is_funny` value.

> [!WARNING] I have not vetted this dataset for appropriateness. While the "not safe for work" label is False for all features, there is still likely to be the usual degeneration one encounters on Reddit.

Many of the posts have `[removed]`, `[deleted]`, and `NaN` for the body text field. This is still information! Aside from replacing `NaN` with an empty string, you probably want to keep these features.

## Deliverables
Your repo should have the following:
1. Your notebook(s) and/or Python scripts where you did your experiments, with the final training run and evaluation rendered
2. A report describing your experiments and your final model decisions
3. An inference script with a `classify` function that loads your model, performs any preprocessing necessary on the list of titles given as inputs, then returns a list of booleans containing the predicted funniness. If your model file(s) is too big for GitHub, you may share a link to it on Google Drive.

### Resources
GPU resources are a challenge, and potentially more important with text data. Here are a few options to consider:
- [Kaggle](https://www.kaggle.com/code) provides 30 hrs/week of free GPU usage
- [Colab](https://colab.research.google.com/signup) provides a "pay as you go" tier, which is $14 for 100 compute units with 90 day expiration. Colab Pro is also an option at $14/month. I hate asking students to pay for things, but think of it like the cost of a textbook. I'm testing out the pay as you go option, and it seems to be using up credits at roughly 1.5/hr on a T4 TPU.

### Your training notebook
I've included some sample code in the `starter.ipynb` to get you started. If you're not using Colab and/or PyTorch, you might need to massage things a bit more (yes, this ends up being an inordinate amount of time in any data project).

### Your inference script
This time, I'm asking that you implement an inference script similar to assignment 1 in `prod.py`. This should handle your data processing, model class definitions, loading the model weights, etc. You can assume that I will be loading the model weights from the working directory; please use relative paths to your weights file.

### Your report
In a separate document, summarize your experiments, models, observations, reflections, etc. I've provided a template with more details in the starter code (`report.md`), though as usual you aren't limited to the markdown format.

## Marking Scheme
I'll be marking each component on the usual 4-point scale and weighting as follows:

| Component                                               | Weight |
| ------------------------------------------------------- | ------ |
| Model development and experimentation process          | 30%    |
| Model performance and compatibility                    | 20%    |
| Report: reflections                                     | 30%    |
| Report: other stuff                        | 20%    |

> As usual, the raw performance doesn't matter too much provided it behaves "okay"

| Score | Description                                                            |
| ----- | ---------------------------------------------------------------------- |
| 4     | Excellent - thoughtful and creative without any errors or omissions    |
| 3     | Pretty good, but with minor errors or omissions                        |
| 2     | Mostly complete, but with major errors or omissions, lacking in detail |
| 1     | A minimal effort was made, incomplete or incorrect                     |
| 0     | No effort was made, or the submission is plagiarized                   |

