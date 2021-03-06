from django.db import models
from django.contrib.auth.models import AbstractBaseUser, BaseUserManager, PermissionsMixin
from django.conf import settings
from django.contrib.auth.hashers import check_password
#from django.contrib.auth.backends import ModelBackend


class MyUserManager(BaseUserManager):
    def create_user(self, username, password=None):
        """
        Creates and saves a User with the given email, date of
        birth and password.
        """
        if not username:
            raise ValueError('Users must have an username')

        user = self.model(
            username=username,
        )

        user.set_password(password)
        user.save(using=self._db)
        return user

    def create_superuser(self, username, password):
        """
        Creates and saves a superuser with the given email, date of
        birth and password.
        """
        user = self.create_user(
            username=username,
            password=password,
        )
        user.is_admin = True
        user.save(using=self._db)
        return user

class MyUser(AbstractBaseUser, PermissionsMixin):
    username = models.CharField(max_length=100)
    is_active = models.BooleanField(default=True)
    is_admin = models.BooleanField(default=False)
    is_superuser = models.BooleanField(default=False)

    USERNAME_FIELD = 'username'
    
    objects = MyUserManager()
    REQUIRED_FIELDS = []

    def __str__(self):
        return self.username

    # def get_all_permissions(self, user_obj, obj=None):
    #     if not user_obj.is_active or user_obj.is_anonymous or obj is not None:
    #         return set()
    #     if not hasattr(user_obj, '_perm_cache'):
    #         user_obj._perm_cache = set()
    #         user_obj._perm_cache.update(self.get_user_permissions(user_obj))
    #         user_obj._perm_cache.update(self.get_group_permissions(user_obj))
    #     return user_obj._perm_cache
    
    #is_superuser = False
    
    # def has_perm(self, perm, obj=None):
    #     "Does the user have a specific permission?"
    #     # Simplest possible answer: Yes, always
    #     return True

    # def has_module_perms(self, app_label):
    #     "Does the user have permissions to view the app `app_label`?"
    #     # Simplest possible answer: Yes, always
    #     return True
    
    # def get_all_permissions(self, obj=None):
    #     pass


class App(models.Model):
    app_name = models.CharField(max_length=50)
    description = models.CharField(max_length=50)
    myuser = models.ForeignKey(settings.AUTH_USER_MODEL, on_delete=models.CASCADE, null=True)

class MyBackend:
    def authenticate(self, request, username=None, password=None):
        try:
            user = MyUser.objects.filter(is_active=True).get(username=username)
            valid_password = check_password(password, user.password)
            if valid_password:
                return user
            else:
                return None
        except MyUser.DoesNotExist:
            return None

    def get_user(self, user_id):
        try:
            return MyUser.objects.filter(is_active=True).get(pk=user_id)
        except MyUser.DoesNotExist:
            return None
