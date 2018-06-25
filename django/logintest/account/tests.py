from django.test import TestCase
from django.test.client import RequestFactory
from django.db import transaction

# Create your tests here.
from .models import App

class TestDB(TestCase):
    def setUp(self):
        a = App.objects.create(app_name='abc', description='bbbbaaa')
        a.save()
        App.objects.create(app_name='dbc')
        print(a)
    
    def test_std(self):
        app = App.objects.get(app_name='abc')
        print(app)