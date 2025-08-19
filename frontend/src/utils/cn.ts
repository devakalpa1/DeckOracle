import { type ClassValue, clsx } from 'clsx'
import { twMerge } from 'tailwind-merge'

/**
 * Merge conditional class strings with proper Tailwind precedence
 * @param inputs - Class values to merge
 * @returns Merged class string
 */
export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}

// Re-export cva for variant definitions
export { cva, type VariantProps } from 'class-variance-authority'
