import { useState } from 'react';
import { useMutation, useQueryClient } from '@tanstack/react-query';
import { api } from '../../services/api';
import { Button } from '../ui/Button';
import { Paperclip, Send, X, FileText, Image as ImageIcon } from 'lucide-react';
import type { Comment, Attachment } from '../../types';
import { format } from 'date-fns';

interface CommentSectionProps {
  taskId: string;
  comments: Comment[];
  attachments: Attachment[];
}

export function CommentSection({ taskId, comments, attachments }: CommentSectionProps) {
  const [newComment, setNewComment] = useState('');
  const [selectedFiles, setSelectedFiles] = useState<File[]>([]);
  const queryClient = useQueryClient();

  const createCommentMutation = useMutation({
    mutationFn: async () => {
      const comment = await api.createComment(taskId, { content: newComment });

      // Upload attachments if any
      if (selectedFiles.length > 0) {
        const formData = new FormData();
        selectedFiles.forEach((file) => {
          formData.append('file', file);
        });
        await api.uploadCommentAttachment(taskId, comment.id, formData);
      }

      return comment;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['comments', taskId] });
      queryClient.invalidateQueries({ queryKey: ['attachments', taskId] });
      setNewComment('');
      setSelectedFiles([]);
    },
  });

  const deleteCommentMutation = useMutation({
    mutationFn: (commentId: string) => api.deleteComment(taskId, commentId),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['comments', taskId] });
    },
  });

  const handleFileSelect = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files) {
      setSelectedFiles([...selectedFiles, ...Array.from(e.target.files)]);
    }
  };

  const handleRemoveFile = (index: number) => {
    setSelectedFiles(selectedFiles.filter((_, i) => i !== index));
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (newComment.trim() || selectedFiles.length > 0) {
      createCommentMutation.mutate();
    }
  };

  // Group attachments by comment_id
  const getCommentAttachments = (commentId: string) => {
    return attachments.filter((att) => att.comment_id === commentId);
  };

  const renderAttachment = (attachment: Attachment) => {
    const isImage = attachment.mime_type.startsWith('image/');

    if (isImage) {
      return (
        <a
          key={attachment.id}
          href={attachment.download_url}
          target="_blank"
          rel="noopener noreferrer"
          className="block w-32 h-32 rounded-lg overflow-hidden border border-slate-200 dark:border-slate-700 hover:border-primary-500 transition-colors"
        >
          <img
            src={attachment.download_url}
            alt={attachment.original_filename}
            className="w-full h-full object-cover"
          />
        </a>
      );
    }

    return (
      <a
        key={attachment.id}
        href={attachment.download_url}
        target="_blank"
        rel="noopener noreferrer"
        className="flex items-center gap-2 px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-700 hover:border-primary-500 hover:bg-slate-50 dark:hover:bg-slate-800 transition-colors"
      >
        <FileText className="w-4 h-4 text-slate-400" />
        <span className="text-sm text-slate-700 dark:text-slate-300 truncate max-w-[200px]">
          {attachment.original_filename}
        </span>
      </a>
    );
  };

  return (
    <div className="space-y-4">
      {/* Comments List */}
      <div className="space-y-3">
        {comments.length === 0 ? (
          <p className="text-sm text-slate-500 dark:text-slate-400 text-center py-4">
            No comments yet. Be the first to comment!
          </p>
        ) : (
          comments.map((comment) => {
            const commentAttachments = getCommentAttachments(comment.id);

            return (
              <div
                key={comment.id}
                className="bg-slate-50 dark:bg-slate-800 rounded-lg p-3 border border-slate-200 dark:border-slate-700"
              >
                <div className="flex items-start justify-between gap-2 mb-2">
                  <div className="flex-1">
                    <p className="text-sm text-slate-900 dark:text-slate-100 whitespace-pre-wrap">
                      {comment.content}
                    </p>
                  </div>
                  <button
                    onClick={() => deleteCommentMutation.mutate(comment.id)}
                    className="text-slate-400 hover:text-red-600 dark:hover:text-red-400 transition-colors"
                  >
                    <X className="w-4 h-4" />
                  </button>
                </div>

                {/* Comment Attachments */}
                {commentAttachments.length > 0 && (
                  <div className="flex flex-wrap gap-2 mt-2">
                    {commentAttachments.map(renderAttachment)}
                  </div>
                )}

                <p className="text-xs text-slate-500 dark:text-slate-400 mt-2">
                  {format(new Date(comment.created_at), 'MMM d, yyyy h:mm a')}
                </p>
              </div>
            );
          })
        )}
      </div>

      {/* New Comment Form */}
      <form onSubmit={handleSubmit} className="border-t border-slate-200 dark:border-slate-700 pt-4">
        <div className="space-y-3">
          <textarea
            value={newComment}
            onChange={(e) => setNewComment(e.target.value)}
            placeholder="Add a comment..."
            rows={3}
            className="w-full px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-900 text-slate-900 dark:text-slate-100 placeholder:text-slate-400 focus:outline-none focus:ring-2 focus:ring-primary-500/20 focus:border-primary-500 text-sm resize-none"
          />

          {/* Selected Files Preview */}
          {selectedFiles.length > 0 && (
            <div className="flex flex-wrap gap-2">
              {selectedFiles.map((file, index) => (
                <div
                  key={index}
                  className="flex items-center gap-2 px-3 py-1.5 bg-slate-100 dark:bg-slate-800 rounded-lg text-sm"
                >
                  {file.type.startsWith('image/') ? (
                    <ImageIcon className="w-4 h-4 text-slate-500" />
                  ) : (
                    <FileText className="w-4 h-4 text-slate-500" />
                  )}
                  <span className="text-slate-700 dark:text-slate-300 truncate max-w-[150px]">
                    {file.name}
                  </span>
                  <button
                    type="button"
                    onClick={() => handleRemoveFile(index)}
                    className="text-slate-400 hover:text-red-600 dark:hover:text-red-400"
                  >
                    <X className="w-3 h-3" />
                  </button>
                </div>
              ))}
            </div>
          )}

          {/* Actions */}
          <div className="flex items-center justify-between">
            <label className="cursor-pointer">
              <input
                type="file"
                multiple
                onChange={handleFileSelect}
                className="hidden"
                accept="image/*,.pdf,.doc,.docx,.txt"
              />
              <div className="flex items-center gap-2 px-3 py-1.5 text-sm text-slate-600 dark:text-slate-400 hover:text-primary-600 dark:hover:text-primary-400 transition-colors">
                <Paperclip className="w-4 h-4" />
                <span>Attach files</span>
              </div>
            </label>

            <Button
              type="submit"
              size="sm"
              disabled={!newComment.trim() && selectedFiles.length === 0}
              isLoading={createCommentMutation.isPending}
            >
              <Send className="w-4 h-4" />
              <span className="ml-1.5">Send</span>
            </Button>
          </div>
        </div>
      </form>
    </div>
  );
}
