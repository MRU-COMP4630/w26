# RNN Activity: Predicting the stock market
![](https://imgs.xkcd.com/comics/marketwatch.png)

> [!NOTE] I do not recommend making any kind of financial decisions based on RNNs.

## Setup
To fetch the data, we'll need the `yfinance` package. Activate your virtual environment, then run `pip install yfinance`. The [starter notebook](rnn_starter.ipynb) shows how to download some data.

## Steps
1. As usual, we need to split the data. For time series data, it is very important that you split **chronologically**, not randomly. The `yfinance` data has a `DateTimeIndex`, which means that we can index it with dates directly, e.g:
   ```python
    train_end = '2023-12-31'
    train = data['Close'][:train_end].values
    ```
    This also assumes that we're interested in just the daily closing price. I'd suggest at least a year's worth of data for both test and validation, with the rest for training.
2. Decide on a **scaling factor** and rescale your data to the 1-ish range. This should not be informed by the test or validation data!
3. Follow the various TODO items to get the single-prediction RNN working.
4. Modify both the `TimeSeriesDataset` and the `SimpleRnnModel` to predict **multiple days** in the future instead of just one.
   > [!TIP] You may need to `squeeze` and/or `expand` your data at some point in the process to ensure the data dimensions are correct

## Further Exercises and Questions
1. What impact does the scaling have on the model? Can you trigger exploding/vanishing gradients?
2. Can you improve the model? Try playing around with LSTM/GRU instead of RNN, number of hidden units, activation functions, etc
3. What happens if you pass a longer time window to the model? You will need to create a new `torch.FloatTensor` from the numpy data to do this instead of using the `TimeSeriesDataset` class.