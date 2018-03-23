from django.urls import path

from . import views

urlpatterns = [
    path('register', views.register, name='register'),
    path('login', views.mylogin, name='authenticate'),
    path('logout', views.mylogout, name='logout'),
]