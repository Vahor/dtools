// From https://losefor.medium.com/implementing-image-caching-with-tauri-enhancing-performance-and-offline-access-6a55c2dbc802

import { exists, BaseDirectory, readFile, writeFile, mkdir } from '@tauri-apps/plugin-fs';
import { fetch } from '@tauri-apps/plugin-http';
const CACHE_DIR = "cache";

/**
 * Cache an image from a given URL to the specified cache directory.
 *
 * @param {string} imageUrl - The URL of the image to be cached.
 * @param {string} [cacheFolder] - The name of the cache folder to use. (`cache/cacheFolder/imageName`)
 * @returns {Promise<void>} A promise that resolves when the image is successfully cached.
 * @throws {Error} If there's an error fetching, creating the cache directory, or writing the image data.
 */
const cacheImage = async (imageUrl: string, cacheFolder?: string): Promise<void> => {
  try {
    const imageData = await fetchImageData(imageUrl);
    const imageName = getImageName(imageUrl);
    const imagePath = getImagePath(imageName, cacheFolder);

    await createCacheDirectory(cacheFolder);
    await writeImageDataToCache(imagePath, imageData);

    console.log("Image cached successfully:", imagePath);
  } catch (error) {
    console.error("Error caching image:", error);
  }
};

/**
 * Fetch image data from the provided URL.
 *
 * @param {string} imageUrl - The URL of the image to fetch.
 * @returns {Promise<ArrayBuffer>} A promise that resolves to the fetched image data.
 */
const fetchImageData = async (imageUrl: string): Promise<ArrayBuffer> => {
  try {
    const response = await fetch(imageUrl).then((res) => res.arrayBuffer());
    return response;
  } catch (error) {
    throw new Error("Error fetching image data: " + error);
  }
};

/**
 * Extracts the image name from the provided URL.
 *
 * @param {string} imageUrl - The URL of the image.
 * @returns {string} The extracted image name.
 */
const getImageName = (imageUrl: string): string => {
  return imageUrl.substring(imageUrl.lastIndexOf("/") + 1);
};

/**
 * Generates the full path to the cache directory.
 *
 * @param {string} imageName - The name of the image.
 * @param {string} [cacheFolder] - The name of the cache folder to use. (`cache/cacheFolder/imageName`)
 * @returns {string} The full path to the cache directory.
 */
const getImagePath = (imageName: string, cacheFolder?: string): string => {
  if (cacheFolder) {
    return `${CACHE_DIR}/${cacheFolder}/${imageName}`;
  }
  return `${CACHE_DIR}/${imageName}`;
};

/**
 * Creates the cache directory if it doesn't exist.
 *
 * @returns {Promise<void>} A promise that resolves when the cache directory is created.
 */
const createCacheDirectory = async (cacheFolder?: string): Promise<void> => {
  try {
    const dir = cacheFolder ? `${CACHE_DIR}/${cacheFolder}` : CACHE_DIR;
    await mkdir(dir, {
      recursive: true,
      baseDir: BaseDirectory.AppData,
    });
  } catch (error) {
    throw new Error("Error creating cache directory: " + error);
  }
};

/**
 * Writes image data to the cache directory.
 *
 * @param {string} imagePath - The path to the image file.
 * @param {ArrayBuffer} imageData - The image data to write.
 * @returns {Promise<void>} A promise that resolves when the image data is written to the cache.
 */
const writeImageDataToCache = async (
  imagePath: string,
  imageData: ArrayBuffer
): Promise<void> => {
  try {
    await writeFile(imagePath, new Uint8Array(imageData), {
      baseDir: BaseDirectory.AppData,
    });
  } catch (error) {
    throw new Error("Error writing image data to cache: " + error);
  }
};

/**
 * Display a cached image or cache and display a new image.
 *
 * @param {string} imageUrl - The URL of the image to be displayed or cached.
 * @param {string} [cacheFolder] - The name of the cache folder to use. (`cache/cacheFolder/imageName`)
 * @returns {Promise<string>} A promise that resolves to a base64-encoded image data URI or the original image URL.
 * @throws {Error} If there's an error reading or caching the image.
 * @example
 * const imageUrl = "https://example.com/image.jpg";
 * const cachedImage = await displayCachedImage(imageUrl);
 * console.log(cachedImage); // Outputs a base64-encoded image data URI or the original image URL.
 */
export const displayCachedImage = async (imageUrl: string, cacheFolder?: string): Promise<string> => {
  const imageName = getImageName(imageUrl);

  const imagePath = getImagePath(imageName, cacheFolder);

  const imageExists = await exists(imagePath, {
    baseDir: BaseDirectory.AppData,
  });

  if (imageExists) {
    // Read the binary file
    const u8Array = await readFile(imagePath, {
      baseDir: BaseDirectory.AppData,
    });

    // Convert to base64 to consume it in the image tag
    const base64Image = _arrayBufferToBase64(u8Array);

    return base64Image;
  } else {
    // Cache the image
    cacheImage(imageUrl, cacheFolder);
    return imageUrl;
  }
};

/**
 * Converts a Uint8Array to a base64-encoded Data URI.
 *
 * @param {Uint8Array} uint8Array - The Uint8Array to convert to base64.
 * @returns {string} A Data URI in the format "data:image/jpg;base64,<base64String>".
 * @example
 * const byteArray = new Uint8Array([255, 216, 255, 224, 0, 16, 74, 70, ...]);
 * const dataUri = _arrayBufferToBase64(byteArray);
 * console.log(dataUri); // Outputs a base64-encoded Data URI.
 */

function _arrayBufferToBase64(uint8Array: Uint8Array): string {
  // Assuming 'uint8Array' is your Uint8Array
  const base64String = btoa(String.fromCharCode(...uint8Array));

  return `data:image/jpg;base64,${base64String}`;
}
