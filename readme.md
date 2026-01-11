# **turn\_into 🎨**

**High-Performance Frequency-Based Color Style Transfer**

turn\_into is a Rust-based CLI tool that maps the aesthetic "soul" of one image onto another. Unlike standard color quantizers that map by visual similarity, this engine uses **statistical frequency mapping** in the **CIE $L^\*a^\*b^\*$** color space to ensure even contrasting styles are transferred effectively.

## ---

**🖼 Examples**


### **Source Image (A)**

![Color Transfer Demo](./images/A.jpg)



### **Style Image (B)**

![Color Transfer Demo](./images/B.jpg)



### **Result**

![Color Transfer Demo](./images/result.png)



## ---

**🚀 Key Features**

* **Frequency-Based Mapping**: Maps colors based on their prevalence (importance) rather than just proximity, allowing for dramatic stylistic shifts.  
* **Perceptual Accuracy**: Operates in the **LAB color space** to ensure quantization matches human visual perception.  
* **Massive Parallelism**: Leverages **Rayon** to utilize every logical core of your CPU, optimized for high-performance chips like the **Intel i7-12700F**.  
* **Optimized Analysis**: Uses thumbnail-based sampling to perform K-Means clustering in milliseconds.

## ---

**🛠 Installation & Usage**

### **Prerequisites**

Ensure you have the Rust toolchain installed. On Arch Linux:

Bash

sudo pacman \-S rust

### **Build**

For image processing, **always** build with the \--release flag to enable compiler optimizations:

Bash

cargo build \--release

### **Run**

The tool takes three required arguments and an optional fourth for the palette size ($K$).

Bash

\# Basic usage (defaults to K=8)  
./target/release/turn\_into ./images/A.jpg ./images/B.jpg ./images/result.png

\# Custom palette size (K=16)  
./target/release/turn\_into ./images/A.jpg ./images/B.jpg ./images/result.png 16

## ---

**🧠 How it Works**

1. **Palette Extraction**: The engine extracts $K$ clusters from both images using the **K-Means** algorithm.  
2. **Statistical Sorting**: Both palettes are sorted by "weight" (pixel count). This identifies which colors are the most dominant in each scene.  
3. **Index Mapping**: Instead of finding the "closest" color, the most common color in Image A is replaced by the most common color in Image B. This allows a purple sunset (most common in source) to be replaced by an anime-style green (most common in style).  
4. **Parallel Reconstruction**: The final pixel buffer is computed in parallel across your CPU's thread pool using a parallel bridge.

## ---

**📈 Performance Optimizations**

* **Delta-E Squared**: We use squared Euclidean distance in the hot loop to avoid expensive sqrt() operations.  
* **Thread Bridging**: Uses par\_bridge() to efficiently distribute image pixel mutation across multiple CPU threads.  
* **Low Memory Footprint**: Analyzes 128x128 thumbnails for palette generation while processing the full-resolution buffer for the final output.

## ---

**🤝 Contributing**

I am a Computer Science student and backend developer. This project is a part of my exploration into high-performance systems and AI-driven media manipulation. I am currently exploring:

* Implementing a GPU compute path via wgpu for my **RTX 5070**.  
* Moving from LAB to **Oklab** for even better hue consistency.  
* Adding support for **Hald CLUT** extraction to save and reuse styles.

### ---

**Author**

**Soso (SosoTaE)** *Batumi, Georgia*

Backend Developer | CS Student