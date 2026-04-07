---
marp: true
paginate: true
theme: marp-theme
math: true
title: Random extras
---

<!-- 
_class: invert lead
_paginate: skip
 -->

# Random extras

COMP 4630 | Winter 2026
Charlotte Curtis

---

## Overview

- Object detection
- Autoencoders
- Generative adversarial networks (GANs)
- Diffusion models
- References and suggested reading:
    - [Scikit-learn book](https://librarysearch.mtroyal.ca/discovery/fulldisplay?context=L&vid=01MTROYAL_INST:02MTROYAL_INST&search_scope=MRULibrary&isFrbr=true&tab=MRULibraryResources&docid=alma9923265933604656): Chapter 17
    - [Deep Learning book](https://www.deeplearningbook.org/): Chapter 14, 20

---

## Object Detection

<div class="columns">
<div>

### R(egion)-CNN
![](../figures/13-rcnn.png)

<span style="font-size:0.8em">Fig 12-25 from Scikit-learn with Pytorch

</div>
<div>

### You Only Look Once (YOLO)
![](../figures/13-yolo.png)

<span style="font-size:0.8em">Fig 1 from [Redmon et al](https://arxiv.org/abs/1506.02640)

</div>
</div>

---

## Autoencoders
- Like encoder-decoder networks, but the input and output are the same, minimizing:
    $$\mathcal{L}(\mathbf{x}, f(g(\mathbf{x})))$$
    where $f$ is the encoder and $g$ is the decoder
- Autoencoders have been around [since the 80s](https://link.springer.com/article/10.1007/BF00332918)
- ❓ If we're just training a model to copy its input, what's the point?

---

## What can we do with autoencoders?
- Autoencoders learn a **latent space** representation of the data, provided the latent space is **lower dimension** than the input (undercomplete)
- **Regularization** can also help the model learn a useful representation
- Once trained, the model can be used for:
   - Data compression
   - Information retrieval
   - Anomaly detection
   - Unsupervised pretraining

---

## Unsupervised pretraining
![bg right fit](../figures/12-fig-17-6.png)
- Rather than training (e.g.) a classifier from scratch, we can first train an **autoencoder**, then use the **encoder** part as the first half of the classifier

<footer>Figure 17-6 from the Scikit-learn book</footer>

---

<!-- ## More autoencoder tricks
- **Sparse autoencoders** introduce a sparsity penalty to the loss function:
    $$\mathcal{L}(\mathbf{x}, g(f(\mathbf{x}))) + \Omega(\mathbf{h})$$
    where $\mathbf{h} = f(\mathbf{x})$ and $\Omega$ is a sparsity penalty
- **Denoising autoencoders** can be created by adding noise to the input:
    $$\mathcal{L}(\mathbf{x}, g(f(\mathbf{\tilde{x}})))$$
- Autoencoders can also be CNNs, RNNs, or just about any other kind of network!

--- -->

## Variational Autoencoders
![bg left fit](../figures/12-fig-17-11.png)

- Encoder produces a **distribution** $(\mathbf{\mu}, \mathbf{\sigma})$
- Decoder samples from $N(\mu, \sigma)$ to produce the output
- Loss includes a **KL divergence** term to push the distribution towards normal

---

## Generative Adversarial Networks (GANs)
- VAEs can be used as generators (e.g. images), so how can we train them to generate what we actually want?
- In [2014](https://proceedings.neurips.cc/paper_files/paper/2014/hash/5ca3e9b122f61f8f06494c97b1afccf3-Abstract.html), Ian Goodfellow et. al proposed training two networks simultaneously: a **generator** and a **discriminator**
    - The generator tries to produce realistic looking images
    - The discriminator is trained to classify images as real or fake
- Two-phase training loop:
    - Equal numbers of real and fake images used to train discriminator
    - Discriminator weights frozen while generator produces images with label of "real" - backpropagation updates only generator weights

---

## GAN training challenges
- Training is a **zero-sum** game, where each "player" receives a payoff that the discriminator tries to maximize and the generator tries to minimize
- Equilibrium is reached when the discriminator performance is 50%, but **oscillations** can occur and convergence is not guaranteed
- **Mode collapse** happens when the generator figures out that it can produce a single image that fools the discriminator
- Still an active area of research!

<div style="font-size: 18pt;" data-marpit-fragment>

> "And since many factors affect these complex dynamics, GANs are very
sensitive to the hyperparameters: you may have to spend a lot of effort fine-tuning them. In fact, that’s why I used RMSProp rather than Nadam when compiling the models: when using Nadam, I ran into a severe mode collapse." - Aurélien Géron

</div>

---

## Diffusion Models
- First formalized in [2015](https://arxiv.org/abs/1503.03585) by Jascha Sohl-Dickstein et. al
- Not popular until [2020](https://arxiv.org/abs/2006.11239) when Jonathan Ho et. al introduced **DDPM**
- Concept: take an image and gradually add noise until it's unrecognizable, then train a model to reverse the process
    ![center](../figures/12-fig-17-20.png)

---

## CLIP guidance
- DDPM, GANs, and VAEs are all capable of generating images given a **class label** that is part of the training data
- Image generation + NLP = magic
- Contrastive Language-Image Pretraining [CLIP](https://arxiv.org/abs/2103.00020) was trained on 400 million image-caption pairs to predict which caption goes with which image
- Final result is a general purpose model that can go both ways
- Actually [open sourced](https://github.com/OpenAI/CLIP) by OpenAI!

---


<!-- 
_class: invert lead
_paginate: skip
 -->

## That's all, next up is your project presentations!