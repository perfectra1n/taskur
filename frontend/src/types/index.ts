export type TaskStatus = 'todo' | 'inprogress' | 'completed';
export type TaskPriority = 'low' | 'medium' | 'high' | 'urgent';

export interface User {
  id: string;
  email: string;
  created_at: string;
}

export interface AuthResponse {
  token: string;
  user: User;
}

export interface Reminder {
  id: string;
  datetime: string;
  type: 'notification' | 'email';
  message?: string;
}

export interface Task {
  id: string;
  user_id: string;
  title: string;
  description: string | null;
  status: TaskStatus;
  priority: TaskPriority;
  due_date: string | null;
  start_date: string | null;
  end_date: string | null;
  hero_image_id: string | null;
  assigned_to: string[];
  reminders: Reminder[];
  tags: string[];
  position: number;
  created_at: string;
  updated_at: string;
}

export interface CreateTaskRequest {
  title: string;
  description?: string;
  status?: TaskStatus;
  priority?: TaskPriority;
  due_date?: string;
  start_date?: string;
  end_date?: string;
  hero_image_id?: string;
  assigned_to?: string[];
  reminders?: Reminder[];
  tags?: string[];
  list_ids?: string[];
}

export interface UpdateTaskRequest {
  title?: string;
  description?: string;
  status?: TaskStatus;
  priority?: TaskPriority;
  due_date?: string;
  start_date?: string;
  end_date?: string;
  hero_image_id?: string;
  assigned_to?: string[];
  reminders?: Reminder[];
  tags?: string[];
  position?: number;
}

export interface List {
  id: string;
  user_id: string;
  name: string;
  color: string | null;
  icon: string | null;
  position: number;
  created_at: string;
  updated_at: string;
}

export interface CreateListRequest {
  name: string;
  color?: string;
  icon?: string;
}

export interface UpdateListRequest {
  name?: string;
  color?: string;
  icon?: string;
  position?: number;
}

export interface Tag {
  id: string;
  user_id: string;
  name: string;
  color: string | null;
  created_at: string;
}

export interface CreateTagRequest {
  name: string;
  color?: string;
}

export interface Comment {
  id: string;
  task_id: string;
  user_id: string;
  content: string;
  created_at: string;
  updated_at: string;
}

export interface CreateCommentRequest {
  content: string;
}

export interface UpdateCommentRequest {
  content: string;
}

export interface Attachment {
  id: string;
  task_id: string | null;
  comment_id: string | null;
  original_filename: string;
  file_size: number;
  mime_type: string;
  download_url: string;
  created_at: string;
}

export interface TaskFilter {
  status?: TaskStatus;
  priority?: TaskPriority;
  tag?: string;
  list_id?: string;
  search?: string;
}

export interface TeamMember {
  id: string;
  user_id: string;
  name: string;
  email: string | null;
  avatar_url: string | null;
  role: string | null;
  created_at: string;
  updated_at: string;
}

export interface CreateTeamMemberRequest {
  name: string;
  email?: string;
  avatar_url?: string;
  role?: string;
}

export interface UpdateTeamMemberRequest {
  name?: string;
  email?: string;
  avatar_url?: string;
  role?: string;
}
