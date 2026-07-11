import React, { useEffect, useState } from 'react';
import { User } from 'lucide-react';
import { cn } from '../utils';

interface AvatarProps extends React.HTMLAttributes<HTMLDivElement> {
  src?: string;
  alt?: string;
  fallback?: string;
  size?: 'sm' | 'md' | 'lg';
  shape?: 'circle' | 'square';
}

export const Avatar = React.forwardRef<HTMLDivElement, AvatarProps>(
  ({ className, src, alt, fallback, size = 'md', shape = 'square', ...props }, ref) => {
    const sizeClasses = {
      sm: 'w-8 h-8 text-xs',
      md: 'w-10 h-10 text-sm',
      lg: 'w-12 h-12 text-base',
    };

    const shapeClasses = {
      circle: 'rounded-full',
      square: 'rounded-md',
    };

    const [hasError, setHasError] = useState(false);

    // Reset error state when src changes so a new URL gets a fresh attempt.
    useEffect(() => {
      setHasError(false);
    }, [src]);

    const showImage = src && !hasError;
    const fallbackText = fallback || alt?.charAt(0);

    return (
      <div
        ref={ref}
        className={cn(
          'relative flex shrink-0 overflow-hidden bg-gray-700 items-center justify-center text-gray-300',
          sizeClasses[size],
          shapeClasses[shape],
          className
        )}
        {...props}
      >
        {showImage ? (
          <img
            src={src}
            alt={alt}
            className="aspect-square h-full w-full object-cover"
            referrerPolicy="no-referrer"
            onError={() => setHasError(true)}
          />
        ) : (
          fallbackText ? (
            <span>{fallbackText}</span>
          ) : (
            <User className="h-1/2 w-1/2 opacity-60" aria-hidden="true" />
          )
        )}
      </div>
    );
  }
);
Avatar.displayName = 'Avatar';
