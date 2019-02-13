from django.http import JsonResponse
from .models import Interface
# Create your views here.
def get_interface(request, productid):
    interface = Interface.objects.filter(url=productid)
    if interface.exists():
        return JsonResponse({"value":interface[0].value})
    else:
        return JsonResponse({"error":"wrong id"})
        
