import { displayCachedImage } from "@/lib/image-caching";
import { useEffect, useState } from "react";

interface ImageProps extends React.ImgHTMLAttributes<HTMLImageElement> {
  src: string;
  thumbnail?: string;
  cacheFolder?: string;
}

export default function ImageCache({ src, thumbnail, cacheFolder, ...props }: ImageProps) {
  const [image, setImage] = useState("/thumbnail.png");

  useEffect(() => {
    loadImage();
  }, []);

  const loadImage = async () => {
    const loadedImage = await displayCachedImage(src, cacheFolder);
    setImage(loadedImage);
  };

  return (
    <div>
      <img src={image} {...props} />
    </div>
  );
}
