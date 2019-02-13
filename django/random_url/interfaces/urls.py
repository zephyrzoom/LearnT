from django.urls import path
from .models import Interface
from . import views

urlpatterns = [
    path('bla/<productid>', views.get_interface),
]