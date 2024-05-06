import React from 'react'
import { twMerge } from 'tailwind-merge'

interface Props {
  label: string
  className?: string
  leftIcon?: React.ReactNode
}

export const Tag: React.FC<Props> = ({ label, className, leftIcon }) => (
  <span
    className={twMerge(
      'mx-1 inline-flex max-h-6 items-center whitespace-nowrap rounded-md bg-gray-100 px-1 py-1 text-xs font-medium text-gray-600',
      className,
    )}
  >
    {leftIcon && <div className="mr-1">{leftIcon}</div>}
    {label}
  </span>
)
