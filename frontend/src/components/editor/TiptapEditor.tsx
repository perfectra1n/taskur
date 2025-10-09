import { useEditor, EditorContent } from '@tiptap/react';
import StarterKit from '@tiptap/starter-kit';
import Placeholder from '@tiptap/extension-placeholder';
import Link from '@tiptap/extension-link';
import TaskList from '@tiptap/extension-task-list';
import TaskItem from '@tiptap/extension-task-item';
import { Bold, Italic, List, ListOrdered, Link as LinkIcon } from 'lucide-react';
import { clsx } from 'clsx';

interface TiptapEditorProps {
  content: string;
  onChange: (content: string) => void;
  placeholder?: string;
}

export function TiptapEditor({ content, onChange, placeholder = 'Start typing...' }: TiptapEditorProps) {
  const editor = useEditor({
    extensions: [
      StarterKit,
      Placeholder.configure({ placeholder }),
      Link.configure({ openOnClick: false }),
      TaskList,
      TaskItem.configure({ nested: true }),
    ],
    content,
    onUpdate: ({ editor }) => {
      onChange(editor.getHTML());
    },
    editorProps: {
      attributes: {
        class: 'tiptap',
      },
    },
  });

  if (!editor) {
    return null;
  }

  return (
    <div className="border border-gray-300 dark:border-gray-600 rounded-lg overflow-hidden">
      {/* Toolbar */}
      <div className="bg-gray-50 dark:bg-gray-700 border-b border-gray-300 dark:border-gray-600 p-2 flex gap-1">
        <button
          onClick={() => editor.chain().focus().toggleBold().run()}
          className={clsx(
            'p-2 rounded hover:bg-gray-200 dark:hover:bg-gray-600',
            editor.isActive('bold') && 'bg-gray-200 dark:bg-gray-600'
          )}
          type="button"
        >
          <Bold className="w-4 h-4" />
        </button>
        <button
          onClick={() => editor.chain().focus().toggleItalic().run()}
          className={clsx(
            'p-2 rounded hover:bg-gray-200 dark:hover:bg-gray-600',
            editor.isActive('italic') && 'bg-gray-200 dark:bg-gray-600'
          )}
          type="button"
        >
          <Italic className="w-4 h-4" />
        </button>
        <div className="w-px bg-gray-300 dark:bg-gray-600 mx-1" />
        <button
          onClick={() => editor.chain().focus().toggleBulletList().run()}
          className={clsx(
            'p-2 rounded hover:bg-gray-200 dark:hover:bg-gray-600',
            editor.isActive('bulletList') && 'bg-gray-200 dark:bg-gray-600'
          )}
          type="button"
        >
          <List className="w-4 h-4" />
        </button>
        <button
          onClick={() => editor.chain().focus().toggleOrderedList().run()}
          className={clsx(
            'p-2 rounded hover:bg-gray-200 dark:hover:bg-gray-600',
            editor.isActive('orderedList') && 'bg-gray-200 dark:bg-gray-600'
          )}
          type="button"
        >
          <ListOrdered className="w-4 h-4" />
        </button>
        <button
          onClick={() => {
            const url = window.prompt('URL');
            if (url) {
              editor.chain().focus().setLink({ href: url }).run();
            }
          }}
          className={clsx(
            'p-2 rounded hover:bg-gray-200 dark:hover:bg-gray-600',
            editor.isActive('link') && 'bg-gray-200 dark:bg-gray-600'
          )}
          type="button"
        >
          <LinkIcon className="w-4 h-4" />
        </button>
      </div>

      {/* Editor */}
      <EditorContent
        editor={editor}
        className="bg-white dark:bg-gray-800 text-gray-900 dark:text-white"
      />
    </div>
  );
}
