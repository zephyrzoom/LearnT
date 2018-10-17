from django import forms
from .models import Test
from django.forms import ModelForm

class Form(ModelForm):
    class Meta:
        model = Test
        fields = ('upload',)