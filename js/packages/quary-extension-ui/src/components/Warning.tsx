import React from 'react'
import { ExclamationTriangleIcon } from '@heroicons/react/20/solid'

interface Props {
  title: string
}

export const Warning: React.FC<React.PropsWithChildren<Props>> = ({
  children,
  title,
}) => (
  <div className="bg-vscode-inputValidation-warningBackground rounded-md p-4 overflow-auto">
    <div className="flex">
      <div className="flex-shrink-0">
        <ExclamationTriangleIcon
          className="text-vscode-inputValidation-warningBorder h-5 w-5"
          aria-hidden="true"
        />
      </div>
      <div className="ml-3">
        <h3 className="text-sm font-medium text-yellow-800">{title}</h3>
        <div className="text-vscode-inputValidation-warningBorder mt-2 text-sm">
          {children}
        </div>
      </div>
    </div>
  </div>
)
