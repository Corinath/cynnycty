/**
 * Cynnycty Color Palette
 * Primary brand colors used throughout the application
 */

export const colors = {
	primaryRed: '#FF3B30',
	primaryPurple: '#8E44AD',
	primaryCyan: '#00B8D9',
	primaryOrange: '#FF9500',
	primaryCharcoal: '#1C1C1E'
} as const;

// Export individual colors for convenience
export const { primaryRed, primaryPurple, primaryCyan, primaryOrange, primaryCharcoal } = colors;

// Type for color keys
export type ColorKey = keyof typeof colors;
