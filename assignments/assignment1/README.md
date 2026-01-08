# Assignment 1: Data discovery and visualization
Due January 30, 2025 at 5 pm. Reasonable requests for extensions will be granted when requested at least 48 hours before the due date.

You may work in groups of 2 or 3. Click [here](https://classroom.github.com/a/Nd9GuOJw) to create your group on GitHub Classroom and clone the starting code.

## Overview
Real world data is messy and incomplete in unexpected ways. Often, the information you need is in some kind of text field, or in a totally separate database that needs to be merged in.  While I have done some basic filtering of the dataset, the focus of this assignment is on **exploring and preparing the data** for use with a machine learning model. 

After exploring and cleaning your data, you will be training a regressor to predict a numeric value. You may choose any of the regression models from [scikit-learn's supervised learning modules](https://scikit-learn.org/stable/supervised_learning.html), provided inference (prediction) is fairly fast (i.e. don't use nearest neighbours). If you want to use something other than scikit-learn, just let me know - it's probably fine! Again, the main focus of this assignment is the dataset exploration and processing part.

Choosing a model is a whole thing in itself, but don't stress about it too much. Feel free to compare a couple, or consult [this chart](https://scikit-learn.org/stable/machine_learning_map.html) for guidance, but it doesn't cover everything. Notably, decisions trees and random forests are missing, despite being pretty common, easily understood, and high-performing models.

I have set aside a subset of the data to test your model. Model performance will be evaluated and compared using the [Mean Absolute  Error](https://en.wikipedia.org/wiki/Mean_absolute_error). Note that since these are not practice or training datasets, the results may not be very good! As long as you're in a reasonable range (less than about 100% relative error), your grade will not be affected by the prediction performance.

## YYC Housing Data
Download the csv from [Google Drive](https://drive.google.com/file/d/1xJsY7UKrq6iOf5BBZZ65dxJkDeIAOlm6/view?usp=sharing) (it's too big for GitHub!)

[Original Source](https://data.calgary.ca/Government/Historical-Property-Assessments-Parcel-/4ur7-wsgc/about_data)

I have removed some redundant columns and excluded non-residential properties such as parking spots. I've also combined the 2023 and 2024 assessment values and excluded properties where the `roll_number` disappeared from one year to the next (e.g. in the case of a subdivision).

Unlike the California housing dataset example, this contains property details and assessed values for each individual house in Calgary. Your goal will be to try to **predict the change in assessment value** from 2023 to 2024 based on the other columns of the dataset. Be careful though - there are some weird city-specific codes in there. For example, the `sub_property_use` column contains the following codes:

| Code   | Description                  |
| ------ | ---------------------------- |
| RE0100 | Residential Acreage          |
| RE0110 | Detached                     |
| RE0111 | Detached with Backyard Suite |
| RE0120 | Duplex                       |
| RE0121 | Duplex Building              |
| RE0201 | Low Rise Apartment Condo     |
| RE0210 | Low Rise Rental Condo        |
| RE0301 | High Rise Apartment Condo    |
| RE0310 | High Rise Rental Condo       |
| RE0401 | Townhouse                    |
| RE0410 | Townhouse Complex            |
| RE0601 | Collective Residence         |
| RE0800 | Manufactured Home            |

Similarly, the `land_use_designation` refers to the city's [land use zones](https://www.calgary.ca/planning/land-use/districts.html), which restrict the type of building that can be constructed on a given property. These zones are about to become much simpler, but for now the column exists in the dataset. It's up to you to decide how (or if) to use it.

Feel free to get creative! Did odd-numbered houses increase more than even? Does distance from city centre make a difference? Apply your **domain knowledge** to select and transform your features.

## Deliverables
Your assignment should consist of both a .ipynb notebook (committed with cells rendered) for your exploratory analysis and model training, as well as your "production" code providing a `predict` function.

### Exploration and model training notebook
This notebook should follow the general process outlined in class to do the first four steps of the [ML Project Checklist](https://github.com/ageron/handson-mlp/blob/main/ml-project-checklist.md), plus training of a simple model. The emphasis is on the data exploration and preparation rather than the model itself. Model shortlisting and fine-tuning is not required (though it is allowed if you'd like).

Specifically, this notebook should include:
- loading the data
- setting aside a test set (as appropriate for the problem)
- exploratory visualizations, with comments about your observations
- your preprocessing pipeline
- your model training, either with cross-validation or a set-aside validation dataset
- saving your pipeline + model for production

Some guidelines are provided in the template notebook - feel free to modify as desired.

If you try something and then ultimately don't use it, it's fine to leave it in the notebook. I'd like to see things you thought of and then discarded.

> [!NOTE]
> While most of your preprocessing decisions are up to you, please do not **drop any samples**. If you encounter missing values, you can drop the column or impute values, but the number of predicted values must match the number of input samples.

### "Production" code
After deciding on a preprocessing pipeline, training a model, and saving it all to disk, implement the function `predict` in `prod.py`. This function should:
- load the required libraries
- load your model from disk
- apply your preprocessing
- return the predicted values

If you are using Scikit-learn's [`Pipeline`](https://scikit-learn.org/stable/modules/generated/sklearn.pipeline.Pipeline.html) class to combine preprocessing with your regression model, then this function could be as simple as loading your pipeline and returning `pipeline.predict(data)`.

`predict` should take as input a Panda dataframe of the data **with the `re_assessed_value_2024` column removed**, and return a Numpy array of predicted property assessment changes from 2023 to 2024. Make sure not to drop any samples! The length of the output must match the length of the input.

> [!IMPORTANT]
> Make sure that I can run your `prod.py` code! I will be running your code from a master script in a Python 3.12 environment with the packages defined in `requirements.txt` installed. I will also `cd` to your repo and import `prod`. Please ensure:
> - You are loading your model using a **relative path**
> - If you have additional Python dependencies, add them to `requirements.txt`
> - If you want to use a different language altogether, that is okay - just make sure that your dependencies are clearly documented and easy to install on Windows 11.

### Written response
Answer the questions in `README.md`. Point form and short responses are fine! If you really hate Markdown, you can add a PDF instead.

## Tips
1. I recommend creating a virtual environment and installing the packages in `requirements.txt`. This will ensure that your code runs on my system:
   ```bash
   python -m venv venv
   pip install -r requirements.txt
   ```
   on a Mac, use `python3` and `pip3` instead.

   If you use any other packages and want to add them to the requirements list, you can update it with:
   ```bash
   pip freeze > requirements.txt
   ```
   (again with `pip3` if you are a Mac user).
2. The [end-to-end ML project](https://github.com/ageron/handson-mlp/blob/main/02_end_to_end_machine_learning_project.ipynb) from the textbook (presented in a condensed form in class) provides examples of *some* data transformation and visualization techniques, but these do not cover all scenarios. You may need to do some additional research to find the right technique for your dataset - in this case, make sure to **cite your sources** with a comment in your code.
3. I have reserved some data for a friendly competition between groups. You might want to test your `predict` function with your own subset of data to make sure the loading and processing behaves in an isolated environment.
4. Make sure to remove the target column from the dataframe before processing! I will be calling `predict` with a dataframe that has `re_assessed_value_2024` removed.

## Marking Scheme
Each of the following components will be marked on a 4-point scale and weighted.

| Component                                           | Weight |
| --------------------------------------------------- | ------ |
| Data exploration (visualizations, observations)     | 30%    |
| Preprocessing decisions                             | 30%    |
| Model inference works and training approach is good | 20%    |
| Written responses                                   | 15%    |

<br />

| Score | Description                                                            |
| ----- | ---------------------------------------------------------------------- |
| 4     | Excellent - thoughtful and creative without any errors or omissions    |
| 3     | Pretty good, but with minor errors or omissions                        |
| 2     | Mostly complete, but with major errors or omissions, lacking in detail |
| 1     | A minimal effort was made, incomplete or incorrect                     |
| 0     | No effort was made, or the submission is plagiarized                   |
