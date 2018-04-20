from django.urls import path

from . import views

urlpatterns = [
    path('register', views.register, name='register'),
    path('login', views.mylogin, name='authenticate'),
    path('logout', views.mylogout, name='logout'),
    path('set_perm', views.set_permission, name='set_permission'),
    path('get_perm', views.get_permission, name='get_permission'),
    path('has_view_perm', views.has_view_perm, name='has_view_perm'),
    path('test_perm', views.test_perm, name='test_perm'),
    path('change_password', views.change_password, name='change_password'),
    path('add_group', views.add_group, name='add_group'),
    path('user_add_group', views.user_add_group, name='user_add_group'),
    path('add_group_perm', views.add_group_perm, name='add_group_perm'),
    path('upload', views.uploadapp, name='upload')
]