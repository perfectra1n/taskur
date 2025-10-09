import { useState, useRef } from 'react';
import type { DragEvent } from 'react';
import { clsx } from 'clsx';
import { Upload, X } from 'lucide-react';
import { useMutation } from '@tanstack/react-query';
import { api } from '../../services/api';

interface HeroImageUploadProps {
  taskId?: string;
  currentImageId?: string | null;
  currentImageUrl?: string | null;
  onImageUploaded: (attachmentId: string, url: string) => void;
  onImageRemoved: () => void;
  className?: string;
}

export function HeroImageUpload({
  taskId,
  currentImageUrl,
  onImageUploaded,
  onImageRemoved,
  className
}: HeroImageUploadProps) {
  const [isDragging, setIsDragging] = useState(false);
  const [preview, setPreview] = useState<string | null>(currentImageUrl || null);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const uploadMutation = useMutation({
    mutationFn: async (file: File) => {
      if (!taskId) {
        // For new tasks, we'll just create a preview
        const url = URL.createObjectURL(file);
        return { id: 'temp-' + Date.now(), url };
      }

      const formData = new FormData();
      formData.append('file', file);

      // Create a FileList-like object from single file
      const dataTransfer = new DataTransfer();
      dataTransfer.items.add(file);
      const fileList = dataTransfer.files;

      const attachments = await api.uploadAttachment(taskId, fileList);
      const attachment = attachments[0];
      return { id: attachment.id, url: attachment.download_url };
    },
    onSuccess: (data) => {
      setPreview(data.url);
      onImageUploaded(data.id, data.url);
    }
  });

  const handleDragOver = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(true);
  };

  const handleDragLeave = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);
  };

  const handleDrop = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);

    const files = Array.from(e.dataTransfer.files);
    const imageFile = files.find(file => file.type.startsWith('image/'));

    if (imageFile) {
      handleFileUpload(imageFile);
    }
  };

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file && file.type.startsWith('image/')) {
      handleFileUpload(file);
    }
  };

  const handleFileUpload = (file: File) => {
    // Create preview immediately
    const url = URL.createObjectURL(file);
    setPreview(url);

    // Upload file
    uploadMutation.mutate(file);
  };

  const handleRemove = () => {
    setPreview(null);
    onImageRemoved();
    if (fileInputRef.current) {
      fileInputRef.current.value = '';
    }
  };

  const handleClick = () => {
    fileInputRef.current?.click();
  };

  if (preview) {
    return (
      <div className={clsx('relative group', className)}>
        <div className="relative w-full h-48 rounded-lg overflow-hidden bg-gray-100 dark:bg-gray-800">
          <img
            src={preview}
            alt="Hero"
            className="w-full h-full object-cover"
          />
          <div className="absolute inset-0 bg-gradient-to-t from-black/50 to-transparent opacity-0 group-hover:opacity-100 transition-opacity" />
          <button
            type="button"
            onClick={handleRemove}
            className="absolute top-3 right-3 p-2 bg-white/90 dark:bg-gray-800/90 rounded-full shadow-lg opacity-0 group-hover:opacity-100 transition-all hover:bg-white dark:hover:bg-gray-800 hover:scale-110"
          >
            <X className="w-4 h-4 text-gray-700 dark:text-gray-300" />
          </button>
        </div>
        {uploadMutation.isPending && (
          <div className="absolute inset-0 flex items-center justify-center bg-black/50 rounded-lg">
            <div className="text-white text-sm font-medium">Uploading...</div>
          </div>
        )}
      </div>
    );
  }

  return (
    <div className={className}>
      <input
        ref={fileInputRef}
        type="file"
        accept="image/*"
        onChange={handleFileSelect}
        className="hidden"
      />

      <div
        onClick={handleClick}
        onDragOver={handleDragOver}
        onDragLeave={handleDragLeave}
        onDrop={handleDrop}
        className={clsx(
          'relative w-full h-48 rounded-lg border-2 border-dashed transition-all cursor-pointer',
          'flex flex-col items-center justify-center gap-3',
          'hover:border-primary-500 hover:bg-primary-50/50 dark:hover:bg-primary-900/10',
          isDragging
            ? 'border-primary-500 bg-primary-50 dark:bg-primary-900/20'
            : 'border-gray-300 dark:border-gray-600 bg-gray-50 dark:bg-gray-800/50'
        )}
      >
        <div className={clsx(
          'p-4 rounded-full transition-colors',
          isDragging
            ? 'bg-primary-100 dark:bg-primary-900/30'
            : 'bg-gray-100 dark:bg-gray-700'
        )}>
          <Upload className={clsx(
            'w-8 h-8 transition-colors',
            isDragging
              ? 'text-primary-600 dark:text-primary-400'
              : 'text-gray-400 dark:text-gray-500'
          )} />
        </div>

        <div className="text-center px-4">
          <p className="text-sm font-medium text-gray-700 dark:text-gray-300">
            Drop an image here or click to upload
          </p>
          <p className="text-xs text-gray-500 dark:text-gray-400 mt-1">
            PNG, JPG, GIF up to 10MB
          </p>
        </div>
      </div>

      {uploadMutation.isError && (
        <p className="mt-2 text-sm text-red-600 dark:text-red-400">
          Failed to upload image. Please try again.
        </p>
      )}
    </div>
  );
}
