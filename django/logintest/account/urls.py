from django.urls import path

from . import views

urlpatterns = [
    path('register', views.register, name='register'),
    path('login', views.mylogin, name='authenticate'),
    path('logout', views.mylogout, name='logout'),
    path('set_perm', views.set_permission, name='set_permission'),
    path('get_perm', views.get_permission, name='get_permission'),
    path('has_view_perm', views.has_view_perm, name='has_view_perm'),
]