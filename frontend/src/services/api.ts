import type {
  AuthResponse,
  Task,
  CreateTaskRequest,
  UpdateTaskRequest,
  List,
  CreateListRequest,
  UpdateListRequest,
  Tag,
  CreateTagRequest,
  Comment,
  CreateCommentRequest,
  UpdateCommentRequest,
  Attachment,
  TaskFilter,
  User,
  TeamMember,
  CreateTeamMemberRequest,
  UpdateTeamMemberRequest,
} from '../types';

const API_BASE = '/api';

class ApiClient {
  private getAuthHeader(): HeadersInit {
    const token = localStorage.getItem('auth_token');
    return token ? { Authorization: `Bearer ${token}` } : {};
  }

  private async request<T>(
    endpoint: string,
    options?: RequestInit
  ): Promise<T> {
    const response = await fetch(`${API_BASE}${endpoint}`, {
      ...options,
      headers: {
        'Content-Type': 'application/json',
        ...this.getAuthHeader(),
        ...options?.headers,
      },
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: 'Unknown error' }));
      throw new Error(error.error || `HTTP ${response.status}`);
    }

    if (response.status === 204) {
      return null as T;
    }

    return response.json();
  }

  // Auth
  async register(email: string, password: string): Promise<AuthResponse> {
    return this.request<AuthResponse>('/auth/register', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
  }

  async login(email: string, password: string): Promise<AuthResponse> {
    return this.request<AuthResponse>('/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });
  }

  async getCurrentUser(): Promise<User> {
    return this.request<User>('/auth/me');
  }

  // Tasks
  async getTasks(filter?: TaskFilter): Promise<Task[]> {
    const params = new URLSearchParams();
    if (filter?.status) params.append('status', filter.status);
    if (filter?.priority) params.append('priority', filter.priority);
    if (filter?.tag) params.append('tag', filter.tag);
    if (filter?.list_id) params.append('list_id', filter.list_id);
    if (filter?.search) params.append('search', filter.search);

    const query = params.toString() ? `?${params.toString()}` : '';
    return this.request<Task[]>(`/tasks${query}`);
  }

  async getTask(id: string): Promise<Task> {
    return this.request<Task>(`/tasks/${id}`);
  }

  async createTask(data: CreateTaskRequest): Promise<Task> {
    return this.request<Task>('/tasks', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async updateTask(id: string, data: UpdateTaskRequest): Promise<Task> {
    return this.request<Task>(`/tasks/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async deleteTask(id: string): Promise<void> {
    return this.request<void>(`/tasks/${id}`, {
      method: 'DELETE',
    });
  }

  // Lists
  async getLists(): Promise<List[]> {
    return this.request<List[]>('/lists');
  }

  async getList(id: string): Promise<List> {
    return this.request<List>(`/lists/${id}`);
  }

  async createList(data: CreateListRequest): Promise<List> {
    return this.request<List>('/lists', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async updateList(id: string, data: UpdateListRequest): Promise<List> {
    return this.request<List>(`/lists/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async deleteList(id: string): Promise<void> {
    return this.request<void>(`/lists/${id}`, {
      method: 'DELETE',
    });
  }

  // Tags
  async getTags(): Promise<Tag[]> {
    return this.request<Tag[]>('/tags');
  }

  async createTag(data: CreateTagRequest): Promise<Tag> {
    return this.request<Tag>('/tags', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async deleteTag(id: string): Promise<void> {
    return this.request<void>(`/tags/${id}`, {
      method: 'DELETE',
    });
  }

  // Comments
  async getComments(taskId: string): Promise<Comment[]> {
    return this.request<Comment[]>(`/tasks/${taskId}/comments`);
  }

  async createComment(taskId: string, data: CreateCommentRequest): Promise<Comment> {
    return this.request<Comment>(`/tasks/${taskId}/comments`, {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async updateComment(
    taskId: string,
    commentId: string,
    data: UpdateCommentRequest
  ): Promise<Comment> {
    return this.request<Comment>(`/tasks/${taskId}/comments/${commentId}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async deleteComment(taskId: string, commentId: string): Promise<void> {
    return this.request<void>(`/tasks/${taskId}/comments/${commentId}`, {
      method: 'DELETE',
    });
  }

  // Attachments
  async getAttachments(taskId: string): Promise<Attachment[]> {
    return this.request<Attachment[]>(`/tasks/${taskId}/attachments`);
  }

  async uploadAttachment(taskId: string, files: FileList): Promise<Attachment[]> {
    const formData = new FormData();
    Array.from(files).forEach((file) => {
      formData.append('file', file);
    });

    const token = localStorage.getItem('auth_token');
    const response = await fetch(`${API_BASE}/tasks/${taskId}/attachments`, {
      method: 'POST',
      headers: token ? { Authorization: `Bearer ${token}` } : {},
      body: formData,
    });

    if (!response.ok) {
      throw new Error('Upload failed');
    }

    return response.json();
  }

  async uploadCommentAttachment(taskId: string, commentId: string, formData: FormData): Promise<Attachment[]> {
    const token = localStorage.getItem('auth_token');
    const response = await fetch(`${API_BASE}/tasks/${taskId}/comments/${commentId}/attachments`, {
      method: 'POST',
      headers: token ? { Authorization: `Bearer ${token}` } : {},
      body: formData,
    });

    if (!response.ok) {
      throw new Error('Upload failed');
    }

    return response.json();
  }

  async deleteAttachment(id: string): Promise<void> {
    return this.request<void>(`/attachments/${id}`, {
      method: 'DELETE',
    });
  }

  getAttachmentUrl(id: string): string {
    return `${API_BASE}/attachments/${id}`;
  }

  // Team Members
  async getTeamMembers(): Promise<TeamMember[]> {
    return this.request<TeamMember[]>('/team-members');
  }

  async getTeamMember(id: string): Promise<TeamMember> {
    return this.request<TeamMember>(`/team-members/${id}`);
  }

  async createTeamMember(data: CreateTeamMemberRequest): Promise<TeamMember> {
    return this.request<TeamMember>('/team-members', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async updateTeamMember(id: string, data: UpdateTeamMemberRequest): Promise<TeamMember> {
    return this.request<TeamMember>(`/team-members/${id}`, {
      method: 'PUT',
      body: JSON.stringify(data),
    });
  }

  async deleteTeamMember(id: string): Promise<void> {
    return this.request<void>(`/team-members/${id}`, {
      method: 'DELETE',
    });
  }
}

export const api = new ApiClient();
