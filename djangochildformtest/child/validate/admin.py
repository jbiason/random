from django import forms
from django.contrib import admin
from django.core.exceptions import ValidationError

from .models import Child
from .models import Parent


class ChildInlineFormset(forms.models.BaseInlineFormSet):
    def clean(self):
        sum = 0
        for form in self.forms:
            if (
                form.is_valid()
                and "proportion" in form.cleaned_data
                and not form.cleaned_data["DELETE"]
            ):
                sum += form.cleaned_data["proportion"]
        print(f">>> total proportion: {sum}")

        if sum > 100:
            raise ValidationError("Invalid proportion")


class ChildAdmin(admin.TabularInline):
    model = Child
    formset = ChildInlineFormset


@admin.register(Parent)
class ParentAdmin(admin.ModelAdmin):
    inlines = [ChildAdmin]
