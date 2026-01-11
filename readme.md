# **turn\_into 🎨**

**A High-Performance Color Transfer & Quantization Engine**

turn\_into is a Rust-based CLI tool designed to map the aesthetic "soul" of one image onto another. By using **K-Means clustering** in the **CIE $L^\*a^\*b^\*$ color space**, it extracts dominant palettes and performs high-fidelity color replacement using parallel processing.

## **🚀 Key Features**

* **K-Means Color Quantization**: Accurately extracts the $K$ most dominant colors from any image.  
* **Perceptual Accuracy**: Operates in **LAB color space** to ensure color mapping matches human visual perception rather than raw RGB math.  
* **High Performance**: Leverages **Rayon** for data-parallelism, utilizing all CPU cores for pixel transformation.  
* **Memory Efficient**: Optimized sampling via thumbnailing for palette extraction.

## ---

**🛠 Installation**

Since this is a Rust project, ensure you have cargo installed (on Arch: sudo pacman \-S rust).

Bash

git clone https://github.com/SosoTaE/turn\_into.git  
cd turn\_into  
cargo build \--release

The optimized binary will be located at target/release/turn\_into.

## ---

**📖 Usage**

Run the tool by providing the source image (the one to change), the target image (the color source), and the output path.

Bash

./target/release/turn\_into \<image\_a\> \<image\_b\> \<output\_path\>

### **Example:**

Bash

\# Map the colors of 'sunset.jpg' onto 'cityscape.png'  
./target/release/turn\_into cityscape.png sunset.jpg result.png

## ---

**🧠 How it Works**

1. **Thumbnail Sampling**: To avoid processing millions of pixels for a simple palette, the engine generates a 128x128 thumbnail of both images.  
2. **K-Means Clustering**: The tool runs the K-Means algorithm (using the kmeans\_colors crate) to find the "centroids" of the color clusters in LAB space.  
3. **Euclidean Mapping**: For every pixel in Image A, the engine calculates the squared Euclidean distance to find the closest color in Palette A, then swaps it with the corresponding index in Palette B.  
4. **Parallel Reconstruction**: The final image is reconstructed pixel-by-pixel across multiple threads.

## ---

**📈 Optimization Details**

* **Delta-E Squared**: We use squared distance calculations to avoid expensive square root operations during pixel comparison.  
* **Static Linking**: Compiles to a single, dependency-free binary for easy deployment on Linux/Arch systems.

## ---

**🤝 Contributing**

Feel free to fork this repo and submit PRs. I'm specifically looking for:

* Support for different $K$ values per image.  
* Sorting palettes by frequency/prevalence for more accurate stylistic matching.  
* WebAssembly (WASM) support for browser-based mapping.

### ---

**Author**

**Soso (SosoTaE)** Backend Developer | Computer Science Student

*Batumi, Georgia*