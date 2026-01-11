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
* **Massive Parallelism**: Leverages **Rayon** to utilize every logical core of your CPU (optimized for modern chips like the i7-12700F).  
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

The tool takes three arguments: the source image, the style reference, and the output path.

Bash

./target/release/turn\_into ./images/A.jpg ./images/B.jpg ./images/result.png

## ---

**🧠 How it Works**

1. **Palette Extraction**: The engine extracts $K$ clusters from both images using K-Means.  
2. **Statistical Sorting**: Both palettes are sorted by "weight" (pixel count).  
3. **Index Mapping**: The most common color in Image A is replaced by the most common color in Image B. This is why a purple sky (common in A) can successfully turn into a green background (common in B).  
4. **Parallel Reconstruction**: The final pixel buffer is computed in parallel across your CPU's thread pool.

## ---

**📈 Performance Optimizations**

* **Delta-E Squared**: We use squared Euclidean distance in the hot loop to avoid expensive sqrt() operations.  
* **Thread Bridging**: Uses par\_bridge() to efficiently distribute image rows across multiple CPU threads.  
* **Low Memory Footprint**: Analyzes small thumbnails for palette generation while processing the full-resolution buffer for the final output.

## ---

**🤝 Contributing**

I am a Computer Science student and backend developer. I'm currently exploring:

* Implementing a GPU compute path via wgpu.  
* Adding an automatic "Elbow Method" to calculate the optimal $K$ value dynamically.  
* WASM support for browser-based processing.

### ---

**Author**

**Soso (SosoTaE)** *Batumi, Georgia* Backend Developer | CS Student