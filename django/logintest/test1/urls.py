from django.contrib import admin
from django.urls import path, include
from . import views

urlpatterns = [
    path('upload', views.upload_file),
    path('set_test_name', views.set_text_name),
]